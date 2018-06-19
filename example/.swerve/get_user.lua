print("We're going to the races");

foo = 1 + 1;

r = empty_response();
r:set_status(200);
r:set_header("X-Powered-By", "Swerve");
r:set_header("Content-Type", "text/plain");
r:set_body("This is my only response");

--return r

r = response(200, "application/json", json_encode({ foo = 123 }));

r:unset_body();
r:set_status(204);

return r

-- return response(200, "application/json", '{ "foo": ' .. foo .. ' }')