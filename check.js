client.test("Check 200", () => {
    client.assert(response.status == 200, "Update session cookie in `http-client.private.env.json`")
});