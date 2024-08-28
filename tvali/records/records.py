"""Records."""
from typing import Optional, Any
from .types import Record, ID, Parameters


class SubcomponentRecord(Record):
    """Subcomponent Record."""
    component_event_id: ID

    @classmethod
    def new(
        cls,
        component_event_id: ID,
        name: str,
        parameters: Optional[Parameters] = None
        ) -> 'SubcomponentRecord':
        return super().new(
            name=name,
            parameters=parameters,
            component_event_id=component_event_id
            )


class ComponentRecord(Record):
    """Component Record."""
    subsystem_event_id: ID

    @classmethod
    def new(
        cls,
        subsystem_event_id: ID,
        name: str,
        parameters: Optional[Parameters] = None
        ) -> 'SubcomponentRecord':
        return super().new(
            name=name,
            parameters=parameters,
            subsystem_event_id=subsystem_event_id
            )

    def subcomponent_record(self, name: str, parameters: Optional[Parameters] = None) -> SubcomponentRecord:
        """Create a new subcomponent record.
        
        Args:
            name (str): The name of the subcomponent record.
            parameters (Optional[Parameters], optional): The parameters of the subcomponent record. Defaults to None.
            
        Returns:
            SubcomponentRecord: A new subcomponent record.
        """
        return SubcomponentRecord.new(self.idx, name, parameters=parameters)


class SubsystemRecord(Record):
    """Subsystem Record."""
    system_event_id: ID
    
    @classmethod
    def new(
        cls,
        system_event_id: ID,
        name: str,
        parameters: Optional[Parameters] = None
        ) -> 'SubsystemRecord':
        """Create a new subsystem record.

        Args:
            system_event_id (ID): The ID of the system event.
            name (str): The name of the subsystem record.
            parameters (Optional[Parameters], optional): The parameters of the subsystem record. Defaults to None.

        Returns:
            SubsystemRecord: A new subsystem record.
        """
        return super().new(
            name=name,
            parameters=parameters,
            system_event_id=system_event_id
            )
    
    def component_record(self, name: str, parameters: Optional[Parameters] = None) -> ComponentRecord:
        return ComponentRecord.new(self.idx, name, parameters=parameters)


class SystemRecord(Record):
    """System Record."""
    def subsystem_record(self, name: str, parameters: Optional[Parameters] = None) -> SubsystemRecord:
        """Create a new subsystem record.

        Args:
            name (str): The name of the subsystem record.
            parameters (Optional[Parameters], optional): The parameters of the subsystem record. Defaults to None.

        Returns:
            SubsystemRecord: A new subsystem record.
        """
        return SubsystemRecord.new(self.idx, name, parameters=parameters)
