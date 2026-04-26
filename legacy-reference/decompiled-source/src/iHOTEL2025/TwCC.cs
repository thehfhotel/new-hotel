namespace iHOTEL2025;

internal enum TwCC : short
{
	Success = 0,
	Bummer = 1,
	LowMemory = 2,
	NoDS = 3,
	MaxConnections = 4,
	OperationError = 5,
	BadCap = 6,
	BadProtocol = 9,
	BadValue = 10,
	SeqError = 11,
	BadDest = 12,
	CapUnsupported = 13,
	CapBadOperation = 14,
	CapSeqError = 15,
	Denied = 16,
	FileExists = 17,
	FileNotFound = 18,
	NotEmpty = 19,
	PaperJam = 20,
	PaperDoubleFeed = 21,
	FileWriteError = 22,
	CheckDeviceOnline = 23
}
