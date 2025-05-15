// Convert 'i64' integer to printable string.
// 
// This code is based on the original C implementation from the GNU project,
// which was licensed under the GNU Lesser General Public License.
// 
// The Rust version maintains the same functionality while adhering to Rust's
// safety practices and memory management.

/// Converts an i64 (equivalent to C's intmax_t) to a string representation.
/// 
/// # Arguments
/// * `num` - The integer to convert to a string
/// 
/// # Returns
/// A String containing the decimal representation of the number
pub fn imaxtostr(num: i64) -> String {
    num.to_string()
}