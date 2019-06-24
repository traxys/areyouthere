package main

import (
	log "github.com/sirupsen/logrus"
	"net"
)

const (
	ID = "ZeComputa"
)

func main() {
	listener, err := net.Listen("tcp", ":1526")
	if err != nil {
		log.WithError(err).Panic("error launching the tcp server")
	}
	log.Info("Started the server on port 1526")
	for {
		conn, err := listener.Accept()
		if err != nil {
			log.WithError(err).Error("error in notifing liveliness")
		}
		conn.Write([]byte("Alive and sound\nID: ZeComputa\n"))
		conn.Close()
	}
}
