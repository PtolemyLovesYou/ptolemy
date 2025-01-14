"""Test log."""

import time
from tqdm.auto import tqdm
from ptolemy import Ptolemy

client = Ptolemy(
    base_url="http://localhost:8000",
    observer_url="http://localhost:50051",
    api_key="pt-sk-ij7LX8Y9fX62oVtaXFFj2OiiIVMC4-7mOpRRyj765k6IGlGY2EwHQ7HlBp5T5i8g",
    workspace_name="fofofofo",
    autoflush=False,
    batch_size=1024
    )

N = 3

start = time.time()
for _ in tqdm(list(range(N))):
    sys = client.trace("test_trace", version='1.2.3', environment='dev')
    with sys:
        sys.inputs(
            foo={"bar": "baz"},
            baz=1,
            qux=True,
            test_str="this is a string",
            test_float=0.93
            )
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
