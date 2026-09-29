use std::collections::HashSet;
use std::path::{Path, PathBuf};

use common::tar_ext;

use crate::common::operation_error::OperationResult;
use crate::data_types::manifest::SegmentManifest;
use crate::types::SnapshotFormat;

pub trait SnapshotEntry {
    /// Segment identifier in the snapshot.
    fn segment_id(&self) -> OperationResult<String>;

    /// Take a snapshot of the segment.
    ///
    /// Creates a tar archive of the segment directory into `snapshot_dir_path`.
    /// Uses `temp_path` to prepare files to archive.
    ///
    /// `exclude_pending_logs` skips packing those pending-changes log files. Collection
    /// snapshotting uses this for logs owned by the *active* snapshot proxies: concurrent CoW
    /// updates flush delete halves into those logs while the upserts live only in the shared
    /// write segment (not archived). Packing the deletes without the write segment would drop
    /// points on restore. Orphan logs (other paths under the segment) are still packed.
    fn take_snapshot(
        &self,
        temp_path: &Path,
        tar: &tar_ext::BuilderExt,
        format: SnapshotFormat,
        manifest: Option<&SegmentManifest>,
        exclude_pending_logs: Option<&HashSet<PathBuf>>,
    ) -> OperationResult<()>;

    fn get_segment_manifest(&self) -> OperationResult<SegmentManifest>;
}
