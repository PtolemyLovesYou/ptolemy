# from ptolemy import ApiKeyPermission
from ptolemy._core import test_enum # pylint: disable=no-name-in-module
from ptolemy._core import MyStrEnum

e1_exp = test_enum(MyStrEnum.ENUM1)
e2_exp = test_enum("Enum2")

for i in MyStrEnum:
    print(i)
# for i in my_strenum.MyStrEnum:
#     print(i)

# print(ApiKeyPermission.ReadOnly)
# print(ApiKeyPermission.ReadOnly == "READ_ONLY")

# d = {
#     ApiKeyPermission.ReadOnly: "READ_ONLY"
#     }

# print(d)
