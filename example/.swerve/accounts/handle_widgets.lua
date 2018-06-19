local accountDB = {}
accountDB['fancy-account'] = {
    name = "Fancy Account",
    widgets = { barfoo = "The best widget in town", bipbop = "A really good widget"}
}

accountDB['less-fancy-account'] = {
    name = "Not Quite As Fancy Account",
    widgets = { cooliowidget = "A really good widget that should be respected" }
}

accountDB['unrelated-account'] = {
    name = "John Mysterio's Vault of Wonders",
    widgets = { yes = "You buy widget, yes?", widget = "Widget is good!", flubber = "Green, Ready to Rock" }
}

account = accountDB[params.account_id]

res = empty_response()
res:set_header('Content-Type', 'application/json')

print("[WIDGETS] Looking for:", params.account_id, params.widget_id)

if not (account == nil) then
    print("[WIDGETS] Found", account.name)
    local widget = account.widgets[params.widget_id]
    if not (widget == nil) then
        res:set_status(200)
        res:set_body(json_encode({ widget = widget }))
    else
        res:set_status(404)
        res:set_body(json_encode({ message = "Could not find widget" }))
    end
else
    res:set_status(404)
    res:set_body(json_encode({ message = "Could not find account" }))
end

return res
