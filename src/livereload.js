(() => {
  const url = "ws://localhost:{{PORT}}/dev_server/ws";
  let shouldReload = false;
  let socket = null;

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
          setTimeout(connect, 2000);
          shouldReload = true;
          break;
        default:
          console.error(`Unknown websocket message: ${event.data}`);
      }
    });

    socket.addEventListener("close", (e) => {
      console.log(`Socket closed, attempting to reconnect: ${e.reason}`);
      socket = null;
      shouldReload = true;

      setTimeout(connect, 2000);
    });

    socket.addEventListener("error", (e) => {
      console.error(`Socket error, closing: ${e.message}`);
      shouldReload = true;
      socket.close();
    });
  };

  connect();
})();