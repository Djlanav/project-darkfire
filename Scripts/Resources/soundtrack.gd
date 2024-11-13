extends Resource

## Resource that allows for easier management of AudioStream Resources
class_name SoundTrack


enum TrackType {
	AMBIENCE,
	MUSIC,
	SFX
}


## Associated AudioStream Resource
@export var audio_stream: AudioStream

## Track name. Used to refer to a particualr SoundTrack
@export var track_name: String

## Which audio bus is its default. Unused.
@export var track_type: TrackType

@export_group("Effects")
## Determines if the audio should be sent to an amplifying bus
@export var amplify: bool

## Determines if the audio should be sent to an hard limiter bus
@export var hard_limiter: bool


## Retrieves the SoundTrack's associated AudioStream
func get_audio_stream() -> AudioStream:
	return audio_stream
