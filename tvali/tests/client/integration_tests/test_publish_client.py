"""Test publish."""

import pytest
from tvali.client import Tvali

test_inputs = {
    "inputs": {"baz": "qux"},
    "outputs": {"qux": 1},
    "feedback": {"quux": {"my": "json"}},
    "metadata": {"quuux": "quuuux"},
}


@pytest.mark.asyncio
async def test_publish():
    """
    Test a simple publish flow with a system, subsystem, component, and subcomponent.
    The system, subsystem, component, and subcomponent all log inputs, outputs, feedback, and metadata.
    """
    sys = Tvali.trace(
        "foo",
        parameters={"foo": "bar"},
    )

    async with sys:
        subsys = sys.spawn("bar", parameters={"bar": "baz"})
        subsys.log(**test_inputs)
        async with subsys:
            com = subsys.spawn("baz", parameters={"baz": "qux"})
            com.log(**test_inputs)
            async with com:
                subcom = com.spawn("qux", parameters={"qux": "quux"})
                subcom.log(**test_inputs)
                async with subcom:
                    pass
