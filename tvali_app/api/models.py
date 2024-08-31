from django.db import models


class Event(models.Model):
    class Meta:
        abstract = True

    id = models.UUIDField(primary_key=True)
    name = models.CharField(max_length=128)
    start_time = models.DateTimeField()
    end_time = models.DateTimeField()
    parameters = models.JSONField()
    error_type = models.CharField(max_length=128)
    error_content = models.JSONField()
    version: models.CharField(max_length=128)


class IO(models.Model):
    class Meta:
        abstract = True

    id = models.UUIDField(primary_key=True)
    field_name = models.CharField(max_length=128)
    field_value = models.JSONField()


class Metadata(models.Model):
    class Meta:
        abstract = True

    id = models.UUIDField(primary_key=True)
    field_name = models.CharField(max_length=128)
    field_value = models.CharField(max_length=256)


class SystemEvent(Event):
    """System Event."""
    ...


class SystemDependent(models.Model):
    class Meta:
        abstract = True

    system_event = models.ForeignKey(SystemEvent, on_delete=models.CASCADE, blank=False)


class SystemInput(IO, SystemDependent):
    """System Input."""
    ...


class SystemOutput(IO, SystemDependent):
    """System Output."""
    ...


class SystemFeedback(IO, SystemDependent):
    """System Feedback."""
    ...


class SystemMetadata(Metadata, SystemDependent):
    """System Metadata."""
    ...


class SubsystemEvent(Event, SystemDependent):
    """Subsystem Event."""
    ...


class SubsystemDependent(models.Model):
    class Meta:
        abstract = True

    subsystem_event = models.ForeignKey(SubsystemEvent, on_delete=models.CASCADE, blank=False)  


class SubsystemInput(IO, SubsystemDependent):
    """Subsystem Input."""
    ...


class SubsystemOutput(IO, SubsystemDependent):
    """Subsystem Output."""
    ...


class SubsystemFeedback(IO, SubsystemDependent):
    """Subsystem Feedback."""
    ...


class SubsystemMetadata(Metadata, SubsystemDependent):
    """Subsystem Metadata."""
    ...


class ComponentEvent(Event, SubsystemDependent):
    """Component Event."""
    ...


class ComponentDependent(models.Model):
    class Meta:
        abstract = True

    component_event = models.ForeignKey(ComponentEvent, on_delete=models.CASCADE, blank=False)


class ComponentInput(IO, ComponentDependent):
    """Component Input."""
    ...


class ComponentOutput(IO, ComponentDependent):
    """Component Output."""
    ...


class ComponentFeedback(IO, ComponentDependent):
    """Component Feedback."""
    ...


class ComponentMetadata(Metadata, ComponentDependent):
    """Component Metadata."""
    ...


class SubcomponentEvent(Event, ComponentDependent):
    """Subcomponent Event."""
    ...


class SubcomponentDependent(models.Model):
    class Meta:
        abstract = True

    subcomponent_event = models.ForeignKey(SubcomponentEvent, on_delete=models.CASCADE, blank=False)


class SubcomponentInput(IO, SubcomponentDependent):
    """Subcomponent Input."""
    ...


class SubcomponentOutput(IO, SubcomponentDependent):
    """Subcomponent Output."""
    ...


class SubcomponentFeedback(IO, SubcomponentDependent):
    """Subcomponent Feedback."""
    ...


class SubcomponentMetadata(Metadata, SubcomponentDependent):
    """Subcomponent Metadata."""
    ...
