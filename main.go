package main

import (
	"context"
	"errors"
	"fmt"
	"log"
	"net/http"
	"strings"
	"sync"
	"time"

	pb "github.com/63square/ipasn_demo/proto/lookup"
	"github.com/valyala/fasthttp"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
)

var (
	stream         pb.Lookup_LookupManyClient
	streamOnce     sync.Once
	streamErr      error
	sendMu         sync.Mutex
	pendingReplies = make(map[string]chan *pb.IpResult)
	repliesMu      sync.Mutex
)

func getOrCreateStream() (pb.Lookup_LookupManyClient, error) {
	streamOnce.Do(func() {
		conn, err := grpc.NewClient("127.0.0.1:36841", grpc.WithTransportCredentials(insecure.NewCredentials()))
		if err != nil {
			streamErr = err
			return
		}

		client := pb.NewLookupClient(conn)
		s, err := client.LookupMany(context.Background())
		if err != nil {
			streamErr = err
			return
		}
		stream = s

		// handle messages
		go func() {
			for {
				resp, err := stream.Recv()
				if err != nil {
					log.Printf("stream recv error: %v", err)
					return
				}
				if resp == nil {
					log.Printf("stream result is nil")
					return
				}

				repliesMu.Lock()
				ch, ok := pendingReplies[resp.Ip]
				if ok {
					delete(pendingReplies, resp.Ip)
					ch <- resp
					close(ch)
				}
				repliesMu.Unlock()
			}
		}()
	})

	return stream, streamErr
}

var ErrLookupTimeout = errors.New("timeout waiting for response")

func lookupIp(s grpc.BidiStreamingClient[pb.IpQuery, pb.IpResult], remoteAddr string) (*pb.IpResult, error) {
	respCh := make(chan *pb.IpResult, 1)

	repliesMu.Lock()
	pendingReplies[remoteAddr] = respCh
	repliesMu.Unlock()

	sendMu.Lock()
	err := s.Send(&pb.IpQuery{
		Ip: remoteAddr,
	})
	sendMu.Unlock()

	if err != nil {
		return nil, err
	}

	select {
	case resp := <-respCh:
		return resp, nil
	case <-time.After(5 * time.Second):
		repliesMu.Lock()
		delete(pendingReplies, remoteAddr)
		repliesMu.Unlock()

		return nil, ErrLookupTimeout
	}
}

func requestHandler(ctx *fasthttp.RequestCtx) {
	s, err := getOrCreateStream()
	if err != nil {
		fmt.Print(ctx, "Internal Server Error")
		ctx.SetStatusCode(http.StatusInternalServerError)
		log.Println(err)
		return
	}

	start := time.Now()

	remoteAddr := ctx.RemoteIP().String()

	testIpHeaders := ctx.Request.Header.PeekAll("X-Test-IP")
	if len(testIpHeaders) > 0 {
		remoteAddr = strings.TrimSpace(string(testIpHeaders[0]))
	}

	ipInfo, err := lookupIp(s, remoteAddr)
	if err != nil {
		fmt.Print(ctx, "Internal Server Error")
		ctx.SetStatusCode(http.StatusInternalServerError)
		log.Println(err)
		return
	}

	enlapsed := time.Since(start).Microseconds()

	fmt.Fprintf(ctx, "Looked up %s in %dus : %+v", ipInfo.Ip, enlapsed, ipInfo.Response)
}

func main() {
	// pre-load stream
	_, err := getOrCreateStream()
	if err != nil {
		log.Fatal(err)
	}

	log.Println("Listening on http://127.0.0.1:8080")

	if err := fasthttp.ListenAndServe("127.0.0.1:8080", requestHandler); err != nil {
		log.Fatalf("Error in ListenAndServe: %v", err)
	}
}
