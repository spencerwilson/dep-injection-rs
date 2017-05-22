import sdk

client = sdk.Client()
result = client.add(1, 2)
assert result == 3
