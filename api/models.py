"""Models."""
from sqlalchemy import Column, String, Uuid, TIMESTAMP, JSON, ForeignKey
from sqlalchemy.orm import relationship, Relationship
from .database import Base


class Event(Base):
    """Event abstract model."""
    __abstract__ = True
    
    id = Column(Uuid, primary_key=True, index=True)
    name = Column(String, nullable=False)
    parameters = Column(JSON)
    start_time = Column(TIMESTAMP, nullable=False)
    end_time = Column(TIMESTAMP, nullable=False)
    error_type = Column(String)
    error_content = Column(String)
    version = Column(String)


class IO(Base):
    """IO abstract model."""
    __abstract__ = True

    id = Column(Uuid, primary_key=True, index=True)
    field_name = Column(String, nullable=False)
    field_value = Column(JSON, nullable=False)


class Metadata(Base):
    """IO abstract model."""
    __abstract__ = True

    id = Column(Uuid, primary_key=True, index=True)
    field_name = Column(String, nullable=False)
    field_value = Column(String, nullable=False)


class SubcomponentInput(IO):
    """Subcomponent Input."""
    __tablename__ = "subcomponent_input"

    subcomponent_event_id = Column(Uuid, ForeignKey("subcomponent_event.id", ondelete="CASCADE"))
    subcomponent_event = relationship("SubcomponentEvent", back_populates="subcomponent_input")


class SubcomponentOutput(IO):
    """Subcomponent Output."""
    __tablename__ = "subcomponent_output"

    subcomponent_event_id = Column(Uuid, ForeignKey("subcomponent_event.id", ondelete="CASCADE"))
    subcomponent_event = relationship("SubcomponentEvent", back_populates="subcomponent_output")


class SubcomponentFeedback(IO):
    """Subcomponent Feedback."""
    __tablename__ = "subcomponent_feedback"

    subcomponent_event_id = Column(Uuid, ForeignKey("subcomponent_event.id", ondelete="CASCADE"))
    subcomponent_event = relationship("SubcomponentEvent", back_populates="subcomponent_feedback")


class SubcomponentMetadata(Metadata):
    """Subcomponent Metadata."""
    __tablename__ = "subcomponent_metadata"

    subcomponent_event_id = Column(Uuid, ForeignKey("subcomponent_event.id", ondelete="CASCADE"))
    subcomponent_event = relationship("SubcomponentEvent", back_populates="subcomponent_metadata")


class SubcomponentEvent(Event):
    """Subcomponent Event."""
    __tablename__ = "subcomponent_event"

    component_event_id = Column(Uuid, ForeignKey("component_event.id", ondelete="CASCADE"))

    subcomponent_input = relationship("SubcomponentInput", back_populates="subcomponent_event")
    subcomponent_output = relationship("SubcomponentOutput", back_populates="subcomponent_event")
    subcomponent_feedback = relationship("SubcomponentFeedback", back_populates="subcomponent_event")
    subcomponent_metadata = relationship("SubcomponentMetadata", back_populates="subcomponent_event")
    
    component_event = relationship("ComponentEvent", back_populates="subcomponent_events", uselist=False)

class ComponentInput(IO):
    """Component Input."""
    __tablename__ = "component_input"

    component_event_id = Column(Uuid, ForeignKey("component_event.id", ondelete="CASCADE"))
    component_event = relationship("ComponentEvent", back_populates="component_input")


class ComponentOutput(IO):
    """Component Output."""
    __tablename__ = "component_output"

    component_event_id = Column(Uuid, ForeignKey("component_event.id", ondelete="CASCADE"))
    component_event = relationship("ComponentEvent", back_populates="component_output")


class ComponentFeedback(IO):
    """Component Feedback."""
    __tablename__ = "component_feedback"

    component_event_id = Column(Uuid, ForeignKey("component_event.id", ondelete="CASCADE"))
    component_event = relationship("ComponentEvent", back_populates="component_feedback")


class ComponentMetadata(Metadata):
    """Component Metadata."""
    __tablename__ = "component_metadata"

    component_event_id = Column(Uuid, ForeignKey("component_event.id", ondelete="CASCADE"))
    component_event = relationship("ComponentEvent", back_populates="component_metadata")


class ComponentEvent(Event):
    """Component Event."""
    __tablename__ = "component_event"

    subsystem_event_id = Column(Uuid, ForeignKey("subsystem_event.id", ondelete="CASCADE"))

    component_input = relationship("ComponentInput", back_populates="component_event")
    component_output = relationship("ComponentOutput", back_populates="component_event")
    component_feedback = relationship("ComponentFeedback", back_populates="component_event")
    component_metadata = relationship("ComponentMetadata", back_populates="component_event")

    subsystem_event = relationship("SubsystemEvent", back_populates="component_events")


class SubsystemInput(IO):
    """Subsystem Input."""
    __tablename__ = "subsystem_input"

    subsystem_event_id = Column(Uuid, ForeignKey("subsystem_event.id", ondelete="CASCADE"))
    subsystem_event = relationship("SubsystemEvent", back_populates="subsystem_input")


class SubsystemOutput(IO):
    """Subsystem Output."""
    __tablename__ = "subsystem_output"

    subsystem_event_id = Column(Uuid, ForeignKey("subsystem_event.id", ondelete="CASCADE"))
    subsystem_event = relationship("SubsystemEvent", back_populates="subsystem_output")


class SubsystemFeedback(IO):
    """Subsystem Feedback."""
    __tablename__ = "subsystem_feedback"

    subsystem_event_id = Column(Uuid, ForeignKey("subsystem_event.id", ondelete="CASCADE"))
    subsystem_event = relationship("SubsystemEvent", back_populates="subsystem_feedback")


class SubsystemMetadata(Metadata):
    """Subsystem Metadata."""
    __tablename__ = "subsystem_metadata"

    subsystem_event_id = Column(Uuid, ForeignKey("subsystem_event.id", ondelete="CASCADE"))
    subsystem_event = relationship("SubsystemEvent", back_populates="subsystem_metadata")


class SubsystemEvent(Event):
    """Subsystem Event."""
    __tablename__ = "subsystem_event"

    system_event_id = Column(Uuid, ForeignKey("system_event.id", ondelete="CASCADE"))
    
    subsystem_input = relationship("SubsystemInput", back_populates="subsystem_event")
    subsystem_output = relationship("SubsystemOutput", back_populates="subsystem_event")
    subsystem_feedback = relationship("SubsystemFeedback", back_populates="subsystem_event")
    subsystem_metadata = relationship("SubsystemMetadata", back_populates="subsystem_event")

    pipeline_event = relationship("PipelineEvent", back_populates="subsystem_events")
    component_events = relationship("ComponentEvent", back_populates="subsystem_event")

class SystemInput(IO):
    """System Input."""
    __tablename__ = "system_input"

    system_event_id = Column(Uuid, ForeignKey("system_event.id", ondelete="CASCADE"))
    system_event = relationship("SystemEvent", back_populates="system_input")


class SystemOutput(IO):
    """System Output."""
    __tablename__ = "system_output"

    system_event_id = Column(Uuid, ForeignKey("system_event.id", ondelete="CASCADE"))
    system_event = relationship("SystemEvent", back_populates="system_output")


class SystemFeedback(IO):
    """System Feedback."""
    __tablename__ = "system_feedback"

    system_event_id = Column(Uuid, ForeignKey("system_event.id", ondelete="CASCADE"))
    system_event = relationship("SystemEvent", back_populates="system_feedback")


class SystemMetadata(Metadata):
    """System Metadata."""
    __tablename__ = "system_metadata"

    system_event_id = Column(Uuid, ForeignKey("system_event.id", ondelete="CASCADE"))
    system_event = relationship("SystemEvent", back_populates="system_metadata")


class SystemEvent(Event):
    """System Event."""
    __tablename__ = "system_event"
    
    system_input = relationship("SystemInput", back_populates="system_event")
    system_output = relationship("SystemOutput", back_populates="system_event")
    system_feedback = relationship("SystemFeedback", back_populates="system_event")
    system_metadata = relationship("SystemMetadata", back_populates="system_event")

    subsystem_events = relationship("SubsystemEvent", back_populates="system_events")
