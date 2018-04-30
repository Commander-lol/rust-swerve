print("We're going to the races");

foo = 1 + 1;

r = empty_response();
r:set_status(404);
-- r:set_header("foo", "bar");
-- r:set_body()

return response(200, "application/json", json_encode({ foo = 123 }));

-- return response(200, "application/json", '{ "foo": ' .. foo .. ' }')