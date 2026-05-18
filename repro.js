const res = await fetch("http://localhost:4000/graphql", {
  headers: {
    "content-type": "application/json",
    "foo-bar": "",
  },
  body: '{"query":"query { __typename  }"}',
  method: "POST",
});

console.log(await res.json());
