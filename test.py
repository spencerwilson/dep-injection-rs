import sdk

print "Python: Constructing a Default Client"
client = sdk.Client()

print "Python: Default Client constructed; calling add method"
result = client.add(1, 2)
assert result == 3

print

def log(message):
  print 'CustomPrefix:', message

print "Python: Constructing a Configured Client"
client = sdk.Client(logger=log)
print "Python: Configured Client constructed; calling add method"
result = client.add(1, 2)
assert result == 3
