
use log::{info, warn};

const DefaultSegmentSize : i32 = 10 * 1024 * 1024;

const WALFileExtension : &str = "wal";

// WALFilePrefix is the prefix on all wal segment files.
const WALFilePrefix : &str = "_";

// walEncodeBufSize is the size of the wal entry encoding buffer
const walEncodeBufSize : i32 = 4 * 1024 * 1024;

const float64EntryType : i32  = 1;
const integerEntryType : i32  = 2;
const booleanEntryType : i32  = 3;
const stringEntryType : i32   = 4;
const unsignedEntryType : i32 = 5;

struct WAL {
    currentSegmentID : int
}

fn NewWAL() -> *WAL {
    &WAL { currentSegmentID: (12)};
}