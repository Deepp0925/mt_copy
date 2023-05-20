mod copy;
fn main() {
    let src = "testing/bike.blend1";
    let dst = "/Volumes/PNY 2/test_dst/bike.blend1";
    copy::copy(src, dst).unwrap();
}
