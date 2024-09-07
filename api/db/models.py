"""Models."""

from typing import List
from sqlalchemy import Column, String, Uuid, TIMESTAMP, JSON, ForeignKey, Text
from sqlalchemy.orm import relationship, Mapped
from tvali import SystemLog, SubsystemLog, ComponentLog, SubcomponentLog, Log
from .database import Base


class EventModel(Base):
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


class IOModel(Base):
    """IO abstract model."""

    __abstract__ = True

    id = Column(Uuid, primary_key=True, index=True)
    field_name = Column(String(32), nullable=False)
    field_value = Column(JSON, nullable=False)


class MetadataModel(Base):
    """IO abstract model."""

    __abstract__ = True

    id = Column(Uuid, primary_key=True, index=True)
    field_name = Column(String(32), nullable=False)
    field_value = Column(String(64), nullable=False)


class SubcomponentInputModel(IOModel):
    """Subcomponent Input."""

    __tablename__ = "subcomponent_input"

    subcomponent_event_id = Column(Uuid, ForeignKey("subcomponent_event.id", ondelete="CASCADE"))
    subcomponent_event: Mapped["SubcomponentEventModel"] = relationship(
        back_populates="subcomponent_input"
    )


class SubcomponentOutputModel(IOModel):
    """Subcomponent Output."""

    __tablename__ = "subcomponent_output"

    subcomponent_event_id = Column(Uuid, ForeignKey("subcomponent_event.id", ondelete="CASCADE"))
    subcomponent_event: Mapped["SubcomponentEventModel"] = relationship(
        back_populates="subcomponent_output"
    )


class SubcomponentFeedbackModel(IOModel):
    """Subcomponent Feedback."""

    __tablename__ = "subcomponent_feedback"

    subcomponent_event_id = Column(Uuid, ForeignKey("subcomponent_event.id", ondelete="CASCADE"))
    subcomponent_event: Mapped["SubcomponentEventModel"] = relationship(
        back_populates="subcomponent_feedback"
    )


class SubcomponentMetadataModel(MetadataModel):
    """Subcomponent Metadata."""

    __tablename__ = "subcomponent_metadata"

    subcomponent_event_id = Column(Uuid, ForeignKey("subcomponent_event.id", ondelete="CASCADE"))
    subcomponent_event: Mapped["SubcomponentEventModel"] = relationship(
        back_populates="subcomponent_metadata"
    )


class SubcomponentEventModel(EventModel):
    """Subcomponent Event."""

    __tablename__ = "subcomponent_event"

    component_event_id = Column(Uuid, ForeignKey("component_event.id", ondelete="CASCADE"))

    subcomponent_input: Mapped[List["SubcomponentInputModel"]] = relationship(
        back_populates="subcomponent_event"
    )
    subcomponent_output: Mapped[List["SubcomponentOutputModel"]] = relationship(
        back_populates="subcomponent_event"
    )
    subcomponent_feedback: Mapped[List["SubcomponentFeedbackModel"]] = relationship(
        back_populates="subcomponent_event"
    )
    subcomponent_metadata: Mapped[List["SubcomponentMetadataModel"]] = relationship(
        back_populates="subcomponent_event"
    )

    component_event: Mapped["ComponentEventModel"] = relationship(
        back_populates="subcomponent_events"
    )

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
            inputs=(
                {inp.field_name: inp.field_value for inp in self.subcomponent_input}
                if self.subcomponent_input
                else None
            ),
            outputs=(
                {out.field_name: out.field_value for out in self.subcomponent_output}
                if self.subcomponent_output
                else None
            ),
            feedback=(
                {fb.field_name: fb.field_value for fb in self.subcomponent_feedback}
                if self.subcomponent_feedback
                else None
            ),
            metadata=(
                {md.field_name: md.field_value for md in self.subcomponent_metadata}
                if self.subcomponent_metadata
                else None
            ),
        )


class ComponentInputModel(IOModel):
    """Component Input."""

    __tablename__ = "component_input"

    component_event_id = Column(Uuid, ForeignKey("component_event.id", ondelete="CASCADE"))
    component_event: Mapped["ComponentEventModel"] = relationship(back_populates="component_input")


class ComponentOutputModel(IOModel):
    """Component Output."""

    __tablename__ = "component_output"

    component_event_id = Column(Uuid, ForeignKey("component_event.id", ondelete="CASCADE"))
    component_event: Mapped["ComponentEventModel"] = relationship(
        back_populates="component_output"
    )


class ComponentFeedbackModel(IOModel):
    """Component Feedback."""

    __tablename__ = "component_feedback"

    component_event_id = Column(Uuid, ForeignKey("component_event.id", ondelete="CASCADE"))
    component_event: Mapped["ComponentEventModel"] = relationship(
        back_populates="component_feedback"
    )


class ComponentMetadataModel(MetadataModel):
    """Component Metadata."""

    __tablename__ = "component_metadata"

    component_event_id = Column(Uuid, ForeignKey("component_event.id", ondelete="CASCADE"))
    component_event: Mapped["ComponentEventModel"] = relationship(
        back_populates="component_metadata"
    )


class ComponentEventModel(EventModel):
    """Component Event."""

    __tablename__ = "component_event"

    subsystem_event_id = Column(Uuid, ForeignKey("subsystem_event.id", ondelete="CASCADE"))

    component_input: Mapped[List["ComponentInputModel"]] = relationship(
        back_populates="component_event"
    )
    component_output: Mapped[List["ComponentOutputModel"]] = relationship(
        back_populates="component_event"
    )
    component_feedback: Mapped[List["ComponentFeedbackModel"]] = relationship(
        back_populates="component_event"
    )
    component_metadata: Mapped[List["ComponentMetadataModel"]] = relationship(
        back_populates="component_event"
    )

    subsystem_event: Mapped["SubsystemEventModel"] = relationship(
        back_populates="component_events"
    )
    subcomponent_events: Mapped[List["SubcomponentEventModel"]] = relationship(
        back_populates="component_event"
    )

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
            inputs=(
                {inp.field_name: inp.field_value for inp in self.component_input}
                if self.component_input
                else None
            ),
            outputs=(
                {out.field_name: out.field_value for out in self.component_output}
                if self.component_output
                else None
            ),
            feedback=(
                {fb.field_name: fb.field_value for fb in self.component_feedback}
                if self.component_feedback
                else None
            ),
            metadata=(
                {md.field_name: md.field_value for md in self.component_metadata}
                if self.component_metadata
                else None
            ),
        )


class SubsystemInputModel(IOModel):
    """Subsystem Input."""

    __tablename__ = "subsystem_input"

    subsystem_event_id = Column(Uuid, ForeignKey("subsystem_event.id", ondelete="CASCADE"))
    subsystem_event: Mapped["SubsystemEventModel"] = relationship(back_populates="subsystem_input")


class SubsystemOutputModel(IOModel):
    """Subsystem Output."""

    __tablename__ = "subsystem_output"

    subsystem_event_id = Column(Uuid, ForeignKey("subsystem_event.id", ondelete="CASCADE"))
    subsystem_event: Mapped["SubsystemEventModel"] = relationship(
        back_populates="subsystem_output"
    )


class SubsystemFeedbackModel(IOModel):
    """Subsystem Feedback."""

    __tablename__ = "subsystem_feedback"

    subsystem_event_id = Column(Uuid, ForeignKey("subsystem_event.id", ondelete="CASCADE"))
    subsystem_event: Mapped["SubsystemEventModel"] = relationship(
        back_populates="subsystem_feedback"
    )


class SubsystemMetadataModel(MetadataModel):
    """Subsystem Metadata."""

    __tablename__ = "subsystem_metadata"

    subsystem_event_id = Column(Uuid, ForeignKey("subsystem_event.id", ondelete="CASCADE"))
    subsystem_event: Mapped["SubsystemEventModel"] = relationship(
        back_populates="subsystem_metadata"
    )


class SubsystemEventModel(EventModel):
    """Subsystem Event."""

    __tablename__ = "subsystem_event"

    system_event_id = Column(Uuid, ForeignKey("system_event.id", ondelete="CASCADE"))

    subsystem_input: Mapped[List["SubsystemInputModel"]] = relationship(
        back_populates="subsystem_event"
    )
    subsystem_output: Mapped[List["SubsystemOutputModel"]] = relationship(
        back_populates="subsystem_event"
    )
    subsystem_feedback: Mapped[List["SubsystemFeedbackModel"]] = relationship(
        back_populates="subsystem_event"
    )
    subsystem_metadata: Mapped[List["SubsystemMetadataModel"]] = relationship(
        back_populates="subsystem_event"
    )

    system_event: Mapped["SystemEventModel"] = relationship(back_populates="subsystem_events")
    component_events: Mapped[List["ComponentEventModel"]] = relationship(
        back_populates="subsystem_event"
    )

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


class SystemInputModel(IOModel):
    """System Input."""

    __tablename__ = "system_input"

    system_event_id = Column(Uuid, ForeignKey("system_event.id", ondelete="CASCADE"))
    system_event: Mapped["SystemEventModel"] = relationship(back_populates="system_input")


class SystemOutputModel(IOModel):
    """System Output."""

    __tablename__ = "system_output"

    system_event_id = Column(Uuid, ForeignKey("system_event.id", ondelete="CASCADE"))
    system_event: Mapped["SystemEventModel"] = relationship(back_populates="system_output")


class SystemFeedbackModel(IOModel):
    """System Feedback."""

    __tablename__ = "system_feedback"

    system_event_id = Column(Uuid, ForeignKey("system_event.id", ondelete="CASCADE"))
    system_event: Mapped["SystemEventModel"] = relationship(back_populates="system_feedback")


class SystemMetadataModel(MetadataModel):
    """System Metadata."""

    __tablename__ = "system_metadata"

    system_event_id = Column(Uuid, ForeignKey("system_event.id", ondelete="CASCADE"))
    system_event: Mapped["SystemEventModel"] = relationship(back_populates="system_metadata")


class SystemEventModel(EventModel):
    """System Event."""

    __tablename__ = "system_event"

    system_input: Mapped[List["SystemInputModel"]] = relationship(back_populates="system_event")
    system_output: Mapped[List["SystemOutputModel"]] = relationship(back_populates="system_event")
    system_feedback: Mapped[List["SystemFeedbackModel"]] = relationship(
        back_populates="system_event"
    )
    system_metadata: Mapped[List["SystemMetadataModel"]] = relationship(
        back_populates="system_event"
    )

    subsystem_events: Mapped[List["SubsystemEventModel"]] = relationship(
        back_populates="system_event"
    )

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
