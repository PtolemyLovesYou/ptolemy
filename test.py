

import os
from tqdm.auto import tqdm
import time

os.environ["OBSERVER_HOST"] = "localhost"
os.environ["OBSERVER_PORT"] = "50051"

from ptolemy import PtolemyClient

print('got here')
client = PtolemyClient(
    workspace_id='d01152e4-ea36-493e-9641-5104dd3f7a20',
    autoflush=True,
    batch_size=1024
    )

N = 1000

start = time.time()
for _ in tqdm(list(range(N))):
    sys = client.trace("test_trace", version='1.2.3', environment='dev')
    with sys:
        sys.inputs(foo={"bar": "baz"}, baz=1, qux=True, test_str="this is a string", test_float=0.93)
        subsys = sys.child("sub_trace", version='1.2.3', environment='dev')
        with subsys:
            comp = subsys.child("comp_trace", version='1.2.3', environment='dev')
            with comp:
                subcomp = comp.child("subcomp_trace", version='1.2.3', environment='dev')
                with subcomp:
                    pass

end = time.time()
client.flush()
print(((end - start) / N) * 1000)
