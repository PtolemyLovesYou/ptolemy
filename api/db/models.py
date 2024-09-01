"""Models."""
from typing import List
from sqlalchemy import Column, String, Uuid, TIMESTAMP, JSON, ForeignKey
from sqlalchemy.orm import relationship, Relationship, Mapped
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
    subcomponent_event: Mapped["SubcomponentEventRecord"] = relationship(back_populates="subcomponent_input")


class SubcomponentOutputRecord(IO):
    """Subcomponent Output."""
    __tablename__ = "subcomponent_output"

    subcomponent_event_id = Column(Uuid, ForeignKey("subcomponent_event.id", ondelete="CASCADE"))
    subcomponent_event: Mapped["SubcomponentEventRecord"] = relationship(back_populates="subcomponent_output")

class SubcomponentFeedbackRecord(IO):
    """Subcomponent Feedback."""
    __tablename__ = "subcomponent_feedback"

    subcomponent_event_id = Column(Uuid, ForeignKey("subcomponent_event.id", ondelete="CASCADE"))
    subcomponent_event: Mapped["SubcomponentEventRecord"] = relationship(back_populates="subcomponent_feedback")


class SubcomponentMetadataRecord(Metadata):
    """Subcomponent Metadata."""
    __tablename__ = "subcomponent_metadata"

    subcomponent_event_id = Column(Uuid, ForeignKey("subcomponent_event.id", ondelete="CASCADE"))
    subcomponent_event: Mapped["SubcomponentEventRecord"] = relationship(back_populates="subcomponent_metadata")


class SubcomponentEventRecord(Event):
    """Subcomponent Event."""
    __tablename__ = "subcomponent_event"

    component_event_id = Column(Uuid, ForeignKey("component_event.id", ondelete="CASCADE"))

    subcomponent_input: Mapped[List["SubcomponentInputRecord"]] = relationship(back_populates="subcomponent_event")
    subcomponent_output: Mapped[List["SubcomponentOutputRecord"]] = relationship(back_populates="subcomponent_event")
    subcomponent_feedback: Mapped[List["SubcomponentFeedbackRecord"]] = relationship(back_populates="subcomponent_event")
    subcomponent_metadata: Mapped[List["SubcomponentMetadataRecord"]] = relationship(back_populates="subcomponent_event")
    
    component_event: Mapped["ComponentEventRecord"] = relationship(back_populates="subcomponent_events")

class ComponentInputRecord(IO):
    """Component Input."""
    __tablename__ = "component_input"

    component_event_id = Column(Uuid, ForeignKey("component_event.id", ondelete="CASCADE"))
    component_event: Mapped["ComponentEventRecord"] = relationship(back_populates="component_input")


class ComponentOutputRecord(IO):
    """Component Output."""
    __tablename__ = "component_output"

    component_event_id = Column(Uuid, ForeignKey("component_event.id", ondelete="CASCADE"))
    component_event: Mapped["ComponentEventRecord"] = relationship(back_populates="component_output")


class ComponentFeedbackRecord(IO):
    """Component Feedback."""
    __tablename__ = "component_feedback"

    component_event_id = Column(Uuid, ForeignKey("component_event.id", ondelete="CASCADE"))
    component_event: Mapped["ComponentEventRecord"] = relationship(back_populates="component_feedback")


class ComponentMetadataRecord(Metadata):
    """Component Metadata."""
    __tablename__ = "component_metadata"

    component_event_id = Column(Uuid, ForeignKey("component_event.id", ondelete="CASCADE"))
    component_event: Mapped["ComponentEventRecord"] = relationship(back_populates="component_metadata")


class ComponentEventRecord(Event):
    """Component Event."""
    __tablename__ = "component_event"

    subsystem_event_id = Column(Uuid, ForeignKey("subsystem_event.id", ondelete="CASCADE"))

    component_input: Mapped[List["ComponentInputRecord"]] = relationship(back_populates="component_event")
    component_output: Mapped[List["ComponentOutputRecord"]] = relationship(back_populates="component_event")
    component_feedback: Mapped[List["ComponentFeedbackRecord"]] = relationship(back_populates="component_event")
    component_metadata: Mapped[List["ComponentMetadataRecord"]] = relationship(back_populates="component_event")

    subsystem_event: Mapped["SubsystemEventRecord"] = relationship(back_populates="component_events")
    subcomponent_events: Mapped[List["SubcomponentEventRecord"]] = relationship(back_populates="component_event")


class SubsystemInputRecord(IO):
    """Subsystem Input."""
    __tablename__ = "subsystem_input"

    subsystem_event_id = Column(Uuid, ForeignKey("subsystem_event.id", ondelete="CASCADE"))
    subsystem_event: Mapped["SubsystemEventRecord"] = relationship(back_populates="subsystem_input")


class SubsystemOutputRecord(IO):
    """Subsystem Output."""
    __tablename__ = "subsystem_output"

    subsystem_event_id = Column(Uuid, ForeignKey("subsystem_event.id", ondelete="CASCADE"))
    subsystem_event: Mapped["SubsystemEventRecord"] = relationship(back_populates="subsystem_output")


class SubsystemFeedbackRecord(IO):
    """Subsystem Feedback."""
    __tablename__ = "subsystem_feedback"

    subsystem_event_id = Column(Uuid, ForeignKey("subsystem_event.id", ondelete="CASCADE"))
    subsystem_event: Mapped["SubsystemEventRecord"] = relationship(back_populates="subsystem_feedback")


class SubsystemMetadataRecord(Metadata):
    """Subsystem Metadata."""
    __tablename__ = "subsystem_metadata"

    subsystem_event_id = Column(Uuid, ForeignKey("subsystem_event.id", ondelete="CASCADE"))
    subsystem_event: Mapped["SubsystemEventRecord"] = relationship(back_populates="subsystem_metadata")


class SubsystemEventRecord(Event):
    """Subsystem Event."""
    __tablename__ = "subsystem_event"

    system_event_id = Column(Uuid, ForeignKey("system_event.id", ondelete="CASCADE"))
    
    subsystem_input: Mapped[List["SubsystemInputRecord"]] = relationship(back_populates="subsystem_event")
    subsystem_output: Mapped[List["SubsystemOutputRecord"]] = relationship(back_populates="subsystem_event")
    subsystem_feedback: Mapped[List["SubsystemFeedbackRecord"]] = relationship(back_populates="subsystem_event")
    subsystem_metadata: Mapped[List["SubsystemMetadataRecord"]] = relationship(back_populates="subsystem_event")

    system_event: Mapped["SystemEventRecord"] = relationship(back_populates="subsystem_events")
    component_events: Mapped[List["ComponentEventRecord"]] = relationship(back_populates="subsystem_event")


class SystemInputRecord(IO):
    """System Input."""
    __tablename__ = "system_input"

    system_event_id = Column(Uuid, ForeignKey("system_event.id", ondelete="CASCADE"))
    system_event: Mapped["SystemEventRecord"] = relationship(back_populates="system_input")


class SystemOutputRecord(IO):
    """System Output."""
    __tablename__ = "system_output"

    system_event_id = Column(Uuid, ForeignKey("system_event.id", ondelete="CASCADE"))
    system_event: Mapped["SystemEventRecord"] = relationship(back_populates="system_output")


class SystemFeedbackRecord(IO):
    """System Feedback."""
    __tablename__ = "system_feedback"

    system_event_id = Column(Uuid, ForeignKey("system_event.id", ondelete="CASCADE"))
    system_event: Mapped["SystemEventRecord"] = relationship(back_populates="system_feedback")


class SystemMetadataRecord(Metadata):
    """System Metadata."""
    __tablename__ = "system_metadata"

    system_event_id = Column(Uuid, ForeignKey("system_event.id", ondelete="CASCADE"))
    system_event: Mapped["SystemEventRecord"] = relationship(back_populates="system_metadata")


class SystemEventRecord(Event):
    """System Event."""
    __tablename__ = "system_event"
    
    system_input: Mapped[List["SystemInputRecord"]] = relationship(back_populates="system_event")
    system_output: Mapped[List["SystemOutputRecord"]] = relationship(back_populates="system_event")
    system_feedback: Mapped[List["SystemFeedbackRecord"]] = relationship(back_populates="system_event")
    system_metadata: Mapped[List["SystemMetadataRecord"]] = relationship(back_populates="system_event")

    subsystem_events: Mapped[List["SubsystemEventRecord"]] = relationship(back_populates="system_event")
