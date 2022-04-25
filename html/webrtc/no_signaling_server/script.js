export class WebRTC {

  constructor(uri) {
    this.config = { 'iceServers': [ { "urls": uri } ] };
    this.option = { ordered: false };
    this.channel;
    this.connection;
  }

  setChannel(target) {
    this.channel = target;
    this.channel.onmessage = (event) => console.log('message: ', event.data);
  }

  createConnection() {
    let thisApp = this;
    this.connection = new RTCPeerConnection(this.config);
    this.connection.onicecandidate = (event) => { event.candidate ? console.log('candidate: ', event.candidate) : console.log(thisApp.connection.localDescription.sdp); };
    this.connection.onconnectionstatechange = (event) => console.log('state: ', thisApp.connection.connectionState);
    this.connection.ondatachannel = (event) => thisApp.setChannel( event.channel );
  }

  startPeerConnection() {
    let thisApp = this;
    this.createConnection();
    this.setChannel( this.connection.createDataChannel('example-channel', this.option) );
    this.connection.createOffer().then( (sessionDescription) => thisApp.connection.setLocalDescription(sessionDescription) );
  }

  setRemoteSdp(sdpText) {
    if (this.connection) {
      let answer = new RTCSessionDescription({ type: 'answer', sdp: sdpText });
      this.connection.setRemoteDescription(answer);
    } else {
      let offer = new RTCSessionDescription({ type: 'offer', sdp: sdpText });
      this.createConnection();
      this.connection.setRemoteDescription(offer);
      this.connection.createAnswer().then( (sessionDescription) => this.connection.setLocalDescription(sessionDescription) );
    }
  }

  sendData(data) {
    this.channel.send(data);
  }
}
