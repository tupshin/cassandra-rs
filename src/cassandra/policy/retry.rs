use cassandra::util::Protected;
use cassandra_sys::CassRetryPolicy as _RetryPolicy;
use cassandra_sys::cass_retry_policy_default_new;
use cassandra_sys::cass_retry_policy_downgrading_consistency_new;
use cassandra_sys::cass_retry_policy_fallthrough_new;
use cassandra_sys::cass_retry_policy_free;
use cassandra_sys::cass_retry_policy_logging_new;

/// The selected retry policy
#[derive(Debug)]
pub struct RetryPolicy(*mut _RetryPolicy);

// The underlying C type has no thread-local state, but does not support access
// from multiple threads: https://datastax.github.io/cpp-driver/topics/#thread-safety
unsafe impl Send for RetryPolicy {}

impl Protected<*mut _RetryPolicy> for RetryPolicy {
    fn inner(&self) -> *mut _RetryPolicy { self.0 }
    fn build(inner: *mut _RetryPolicy) -> Self { RetryPolicy(inner) }
}

impl RetryPolicy {
    /// The default retry policy
    pub fn default_new() -> Self { unsafe { RetryPolicy(cass_retry_policy_default_new()) } }

    /// An auto-CL-downgrading consistency level
    pub fn downgrading_consistency_new() -> Self {
        unsafe { RetryPolicy(cass_retry_policy_downgrading_consistency_new()) }
    }

    /// a fallthrough retry policy
    pub fn fallthrough_new() -> Self { unsafe { RetryPolicy(cass_retry_policy_fallthrough_new()) } }

    /// The a logging retry policy
    pub fn logging_new(child_retry_policy: RetryPolicy) -> Self {
        unsafe { RetryPolicy(cass_retry_policy_logging_new(child_retry_policy.0)) }
    }
}

impl Drop for RetryPolicy {
    fn drop(&mut self) {
        unsafe {
            cass_retry_policy_free(self.0);
        }
    }
}
