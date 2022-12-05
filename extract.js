const day = request.url.getRaw().split("/")[3];
request.variables.set("dayN", `day${day}`);