"""Test log."""

import time
from tqdm.auto import tqdm
from ptolemy import Ptolemy

client = Ptolemy(
    base_url="http://localhost:8000",
    observer_url="http://localhost:8000",
    api_key="pt-sk-q1pVe_zjHloua1doiBUYQ15bsAZLn2QZHBBRbgNhmY4N9EU_IKYOELg9aKPMedIC",
    workspace_name="default",
    autoflush=False,
    batch_size=1024
    )

N = 100

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
