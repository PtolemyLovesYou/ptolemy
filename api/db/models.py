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


class SubcomponentInputRecord(IO):
    """Subcomponent Input."""
    __tablename__ = "subcomponent_input"

    subcomponent_event_id = Column(Uuid, ForeignKey("subcomponent_event.id", ondelete="CASCADE"))
    subcomponent_event = relationship("SubcomponentEventRecord", back_populates="subcomponent_input")


class SubcomponentOutputRecord(IO):
    """Subcomponent Output."""
    __tablename__ = "subcomponent_output"

    subcomponent_event_id = Column(Uuid, ForeignKey("subcomponent_event.id", ondelete="CASCADE"))
    subcomponent_event = relationship("SubcomponentEventRecord", back_populates="subcomponent_output")


class SubcomponentFeedbackRecord(IO):
    """Subcomponent Feedback."""
    __tablename__ = "subcomponent_feedback"

    subcomponent_event_id = Column(Uuid, ForeignKey("subcomponent_event.id", ondelete="CASCADE"))
    subcomponent_event = relationship("SubcomponentEventRecord", back_populates="subcomponent_feedback")


class SubcomponentMetadataRecord(Metadata):
    """Subcomponent Metadata."""
    __tablename__ = "subcomponent_metadata"

    subcomponent_event_id = Column(Uuid, ForeignKey("subcomponent_event.id", ondelete="CASCADE"))
    subcomponent_event = relationship("SubcomponentEventRecord", back_populates="subcomponent_metadata")


class SubcomponentEventRecord(Event):
    """Subcomponent Event."""
    __tablename__ = "subcomponent_event"

    component_event_id = Column(Uuid, ForeignKey("component_event.id", ondelete="CASCADE"))

    subcomponent_input = relationship("SubcomponentInputRecord", back_populates="subcomponent_event")
    subcomponent_output = relationship("SubcomponentOutputRecord", back_populates="subcomponent_event")
    subcomponent_feedback = relationship("SubcomponentFeedbackRecord", back_populates="subcomponent_event")
    subcomponent_metadata = relationship("SubcomponentMetadataRecord", back_populates="subcomponent_event")
    
    component_event = relationship("ComponentEventRecord", back_populates="subcomponent_events", uselist=False)

class ComponentInputRecord(IO):
    """Component Input."""
    __tablename__ = "component_input"

    component_event_id = Column(Uuid, ForeignKey("component_event.id", ondelete="CASCADE"))
    component_event = relationship("ComponentEventRecord", back_populates="component_input")


class ComponentOutputRecord(IO):
    """Component Output."""
    __tablename__ = "component_output"

    component_event_id = Column(Uuid, ForeignKey("component_event.id", ondelete="CASCADE"))
    component_event = relationship("ComponentEventRecord", back_populates="component_output")


class ComponentFeedbackRecord(IO):
    """Component Feedback."""
    __tablename__ = "component_feedback"

    component_event_id = Column(Uuid, ForeignKey("component_event.id", ondelete="CASCADE"))
    component_event = relationship("ComponentEventRecord", back_populates="component_feedback")


class ComponentMetadataRecord(Metadata):
    """Component Metadata."""
    __tablename__ = "component_metadata"

    component_event_id = Column(Uuid, ForeignKey("component_event.id", ondelete="CASCADE"))
    component_event = relationship("ComponentEventRecord", back_populates="component_metadata")


class ComponentEventRecord(Event):
    """Component Event."""
    __tablename__ = "component_event"

    subsystem_event_id = Column(Uuid, ForeignKey("subsystem_event.id", ondelete="CASCADE"))

    component_input = relationship("ComponentInputRecord", back_populates="component_event")
    component_output = relationship("ComponentOutputRecord", back_populates="component_event")
    component_feedback = relationship("ComponentFeedbackRecord", back_populates="component_event")
    component_metadata = relationship("ComponentMetadataRecord", back_populates="component_event")

    subsystem_event = relationship("SubsystemEventRecord", back_populates="component_events")
    subcomponent_events = relationship("SubcomponentEventRecord", back_populates="component_event")


class SubsystemInputRecord(IO):
    """Subsystem Input."""
    __tablename__ = "subsystem_input"

    subsystem_event_id = Column(Uuid, ForeignKey("subsystem_event.id", ondelete="CASCADE"))
    subsystem_event = relationship("SubsystemEventRecord", back_populates="subsystem_input")


class SubsystemOutputRecord(IO):
    """Subsystem Output."""
    __tablename__ = "subsystem_output"

    subsystem_event_id = Column(Uuid, ForeignKey("subsystem_event.id", ondelete="CASCADE"))
    subsystem_event = relationship("SubsystemEventRecord", back_populates="subsystem_output")


class SubsystemFeedbackRecord(IO):
    """Subsystem Feedback."""
    __tablename__ = "subsystem_feedback"

    subsystem_event_id = Column(Uuid, ForeignKey("subsystem_event.id", ondelete="CASCADE"))
    subsystem_event = relationship("SubsystemEventRecord", back_populates="subsystem_feedback")


class SubsystemMetadataRecord(Metadata):
    """Subsystem Metadata."""
    __tablename__ = "subsystem_metadata"

    subsystem_event_id = Column(Uuid, ForeignKey("subsystem_event.id", ondelete="CASCADE"))
    subsystem_event = relationship("SubsystemEventRecord", back_populates="subsystem_metadata")


class SubsystemEventRecord(Event):
    """Subsystem Event."""
    __tablename__ = "subsystem_event"

    system_event_id = Column(Uuid, ForeignKey("system_event.id", ondelete="CASCADE"))
    
    subsystem_input = relationship("SubsystemInputRecord", back_populates="subsystem_event")
    subsystem_output = relationship("SubsystemOutputRecord", back_populates="subsystem_event")
    subsystem_feedback = relationship("SubsystemFeedbackRecord", back_populates="subsystem_event")
    subsystem_metadata = relationship("SubsystemMetadataRecord", back_populates="subsystem_event")

    pipeline_event = relationship("PipelineEventRecord", back_populates="subsystem_events")
    component_events = relationship("ComponentEventRecord", back_populates="subsystem_event")


class SystemInputRecord(IO):
    """System Input."""
    __tablename__ = "system_input"

    system_event_id = Column(Uuid, ForeignKey("system_event.id", ondelete="CASCADE"))
    system_event = relationship("SystemEventRecord", back_populates="system_input")


class SystemOutputRecord(IO):
    """System Output."""
    __tablename__ = "system_output"

    system_event_id = Column(Uuid, ForeignKey("system_event.id", ondelete="CASCADE"))
    system_event = relationship("SystemEventRecord", back_populates="system_output")


class SystemFeedbackRecord(IO):
    """System Feedback."""
    __tablename__ = "system_feedback"

    system_event_id = Column(Uuid, ForeignKey("system_event.id", ondelete="CASCADE"))
    system_event = relationship("SystemEventRecord", back_populates="system_feedback")


class SystemMetadataRecord(Metadata):
    """System Metadata."""
    __tablename__ = "system_metadata"

    system_event_id = Column(Uuid, ForeignKey("system_event.id", ondelete="CASCADE"))
    system_event = relationship("SystemEventRecord", back_populates="system_metadata")


class SystemEventRecord(Event):
    """System Event."""
    __tablename__ = "system_event"
    
    system_input = relationship("SystemInputRecord", back_populates="system_event")
    system_output = relationship("SystemOutputRecord", back_populates="system_event")
    system_feedback = relationship("SystemFeedbackRecord", back_populates="system_event")
    system_metadata = relationship("SystemMetadataRecord", back_populates="system_event")

    subsystem_events = relationship("SubsystemEventRecord", back_populates="system_events")
