Given("that my name is ''", function(name)
    ACK.name = name
end)

Then("say hello", function()
    OUT = "Hello, " .. ACK.name .. "!"
end)

Then("print all data", function()
    print(OUT)
end)

ZEN:begin(1)

local script = [[
Given that my name is 'Julian'
Then say hello
And print all data
]]

ZEN:parse(script)
ZEN:run({}, {})

-- print("\n---\n")
-- print(ZEN_traceback)
