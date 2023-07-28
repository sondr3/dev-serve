(() => {
  const url = "ws://localhost:{{PORT}}/dev_server/ws";
  let shouldReload = false;
  let socket = null;

  const exponentialBackoff = (backoff, deadline) => {
    let n = 0;
    let sum = 0;

    return function waitTime() {
      let wait = Math.pow(2, n++);
      wait = Math.min(wait, backoff);
      sum += wait;
      if (sum > deadline) {
        return 6400;
      }
      wait += Math.random();
      return Math.ceil(wait * 1000);
    };
  };

  let backoff = 64;
  let deadline = 600;
  let retryWait = exponentialBackoff(backoff, deadline);

  const connect = () => {
    socket = new WebSocket(url);

    socket.addEventListener("open", (_event) => {
      if (shouldReload) {
        console.log("Reloading page");
        location.reload();
      }

      console.log(`Socket connected`);
    });

    socket.addEventListener("message", (event) => {
      switch (event.data.trim()) {
        case "reload":
          shouldReload = false;
          console.log("Reloading page");
          location.reload();
          break;
        case "shutdown":
          socket.close(1000, "Waiting for server to restart");
          shouldReload = true;

          setTimeout(connect, retryWait());
          break;
        default:
          console.error(`Unknown websocket message: ${event.data}`);
      }
    });

    socket.addEventListener("close", (e) => {
      console.log(`Socket closed, attempting to reconnect: ${e.reason}`);
      shouldReload = true;
      setTimeout(connect, retryWait());
    });

    socket.addEventListener("error", (e) => {
      console.error(`Socket error, closing: ${e.message}`);
      shouldReload = true;
      socket.close();
    });
  };

  connect();
})();
