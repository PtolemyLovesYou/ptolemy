"""Test Records classes."""
from tvali.records.records import SystemRecord, SubsystemRecord, SubcomponentRecord, ComponentRecord

def test_record_creation():
    system_record = SystemRecord.new('system_name')
    subsystem_record = system_record.subsystem_record('subsystem_name')
    component_record = subsystem_record.component_record('component_name')
    subcomponent_record = component_record.subcomponent_record('subcomponent_name')

    assert isinstance(system_record, SystemRecord)
    assert isinstance(subsystem_record, SubsystemRecord)
    assert isinstance(component_record, ComponentRecord)
    assert isinstance(subcomponent_record, SubcomponentRecord)
