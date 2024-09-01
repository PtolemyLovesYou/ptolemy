"""Models."""
from typing import List
from sqlalchemy import Column, String, Uuid, TIMESTAMP, JSON, ForeignKey, Text
from sqlalchemy.orm import relationship, Mapped
from tvali import SystemLog, SubsystemLog, ComponentLog, SubcomponentLog, Log
from .database import Base


class Event(Base):
    """Event abstract model."""
    __abstract__ = True
    
    id = Column(Uuid, primary_key=True, index=True)
    name = Column(String(64), nullable=False)
    parameters = Column(JSON)
    start_time = Column(TIMESTAMP, nullable=False)
    end_time = Column(TIMESTAMP, nullable=False)
    error_type = Column(String(64))
    error_content = Column(Text())
    version = Column(String(32))
    environment = Column(Text())

    def to_log(self) -> Log:
        """
        Abstract method to convert the event to a tvali Log object.
        
        :return: A tvali Log object.
        """
        raise NotImplementedError(f"Event type {self.__class__.__name__} must be defined.")


class IO(Base):
    """IO abstract model."""
    __abstract__ = True

    id = Column(Uuid, primary_key=True, index=True)
    field_name = Column(String(32), nullable=False)
    field_value = Column(JSON, nullable=False)


class Metadata(Base):
    """IO abstract model."""
    __abstract__ = True

    id = Column(Uuid, primary_key=True, index=True)
    field_name = Column(String(32), nullable=False)
    field_value = Column(String(64), nullable=False)


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
    
    def to_log(self) -> SubcomponentLog:
        return SubcomponentLog(
            id=self.id,
            component_event_id=self.component_event_id,
            name=self.name,
            parameters=self.parameters,
            start_time=self.start_time,
            end_time=self.end_time,
            error_type=self.error_type,
            error_content=self.error_content,
            version=self.version,
            environment=self.environment,
            inputs={inp.field_name: inp.field_value for inp in self.subcomponent_input} if self.subcomponent_input else None,
            outputs={out.field_name: out.field_value for out in self.subcomponent_output} if self.subcomponent_output else None,
            feedback={fb.field_name: fb.field_value for fb in self.subcomponent_feedback} if self.subcomponent_feedback else None,
            metadata={md.field_name: md.field_value for md in self.subcomponent_metadata} if self.subcomponent_metadata else None,
        )


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
    
    def to_log(self) -> ComponentLog:
        return ComponentLog(
            id=self.id,
            subsystem_event_id=self.subsystem_event_id,
            name=self.name,
            parameters=self.parameters,
            start_time=self.start_time,
            end_time=self.end_time,
            error_type=self.error_type,
            error_content=self.error_content,
            version=self.version,
            environment=self.environment,
            inputs={inp.field_name: inp.field_value for inp in self.component_input} if self.component_input else None,
            outputs={out.field_name: out.field_value for out in self.component_output} if self.component_output else None,
            feedback={fb.field_name: fb.field_value for fb in self.component_feedback} if self.component_feedback else None,
            metadata={md.field_name: md.field_value for md in self.component_metadata} if self.component_metadata else None,
        )


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
    
    def to_log(self) -> SubsystemLog:
        return SubsystemLog(
            id=self.id,
            system_event_id=self.system_event_id,
            name=self.name,
            parameters=self.parameters,
            start_time=self.start_time,
            end_time=self.end_time,
            error_type=self.error_type,
            error_content=self.error_content,
            version=self.version,
            environment=self.environment,
            inputs=self.subsystem_input,
            outputs=self.subsystem_output,
            feedback=self.subsystem_feedback,
            metadata=self.subsystem_metadata,
        )


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

    def to_log(self) -> SystemLog:
        return SystemLog(
            id=self.id,
            name=self.name,
            parameters=self.parameters,
            start_time=self.start_time,
            end_time=self.end_time,
            error_type=self.error_type,
            error_content=self.error_content,
            inputs=self.system_input,
            outputs=self.system_output,
            feedback=self.system_feedback,
            metadata=self.system_metadata,
        )
