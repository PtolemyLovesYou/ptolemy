from ptolemy import ApiKeyPermission

print(ApiKeyPermission.ReadOnly)
print(ApiKeyPermission.ReadOnly == "READ_ONLY")

d = {
    ApiKeyPermission.ReadOnly: "READ_ONLY"
    }

print(d)

for i in list(ApiKeyPermission):
    print(repr(i), i.value)
