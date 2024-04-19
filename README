# Vid Conference App(In development)

A zoom-like application with additional features to for online class

## TECHNOLOGIES REQUIRED

1. WebRTC(Web Real-Time Communication): a web standard that enables real-time communication between browsers. It provides APIs for capturing audio and video, establishing peer-to-peer connections and exchanging media data.

   - packages: webrtc-rs, webrtc-interop

2. Signaling: a process of establishing a connection between two peers. It involves exchanging sessions descriptions, ICE candidates, and other metadata required for setting up the peer-to-peer connection. Signaling servers such us WebSocket server or a REST API can be used.

   - packages: actix-web, tungstenite

3. Media Capture and Streaming: audio and video from the user's device such as microphone and webcam must be captured. This can be done using MediaDevices API in the browser. Media can then be streamed using WebRTC.

   - packages: media-stream, gstreamer-rs

4. Peer-to-Peer Connection: WebRTC allows direct peer-to-peer connections between browsers. This eliminates the need for a centralized server to relay media data. However, in some cases, we may need to use a TURN server to help peers establish a connection when direct peer-to-peer communication is not possible due to NAT traversal issues.
   TURN(Traversal Using Relays around NAT) server: is a type of server used in peer-to-peer communication, particulary in scenarios where direct peer-to-peer connections are not possible due to network restrictions or NAT(Network Address Translation) traversal issues - packages: webrtc-rs

5. Data Channel: WebRTC provides a Data Channel API that allows you to send arbitrary data between peers. This can be used for sending text messages, file transfers or any other data alongside the audio and video streams.

   - packages: webrtc-rs

6. Codecs and Formats: Appropraite audio and video codecs and formats for encoding and decoding the media data must be chosen. Common codecs include H.264 for video and Opus for audio. Resolution, frame rate and bitrate for the video stream must be considered too.

   - packages: h264-rs, opus-codec, libvpx-rs

7. Network and Bandwidth Management: Video Conferencing requires a stable and fast Network connection. Network conditions such as packat loss and latency must be handles, bitrate and resolution must be adjusted accordingly to provide a smooth user experience. Techniques like congestion control and adaptive bitrate streaming can be employed.

   - packages: tokio, bandwidth

8. Security and Privacy: When dealing with sensitive user data, such as audio and video streams, security and privacy become important considerations. Implement measures like encryption, access control and data protection to safeguard user privacy.
   - packages: sodiumoxide, ring

## TechStack selected(rust packages)

    - yew: A modern Rust framework for building multi-threaded front-end web apps with WebAssembly.
    - actix-web: A powerful, high-performance web framework for Rust will be used to build WebSocket and REST API signaling server.
    - `webrtc-rs`: A pure Rust impl of WebRTC
    - media-stream: for access to media devices
    - gstreamer-rs: for media processing
    # Codecs and Formats
    - h264-rs: Rust H.264 video codec implementation
    - opus-codec: Rust wrapper around the Opus audio codec
    - tokio: A runtime for writing asynchronours applications in Rust will provide networking, concurrency and I/O tasks handling.
