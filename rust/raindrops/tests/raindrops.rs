use raindrops::raindrops;

#[test]
fn the_sound_for_1_is_1() {
    assert_eq!(raindrops(1), "1");
}

#[test]
#[ignore]
fn the_sound_for_3_is_pling() {
    assert_eq!(raindrops(3), "Pling");
}

#[test]
#[ignore]
fn the_sound_for_5_is_plang() {
    assert_eq!(raindrops(5), "Plang");
}

#[test]
#[ignore]
fn the_sound_for_7_is_plong() {
    assert_eq!(raindrops(7), "Plong");
}

#[test]
#[ignore]
fn the_sound_for_15_is_plingplang() {
    assert_eq!(raindrops(15), "PlingPlang");
}

#[test]
#[ignore]
fn the_sound_for_21_is_plingplong() {
    assert_eq!(raindrops(21), "PlingPlong");
}

#[test]
#[ignore]
fn the_sound_for_35_is_plangplong() {
    assert_eq!(raindrops(35), "PlangPlong");
}

#[test]
#[ignore]
fn the_sound_for_105_is_plingplangplong() {
    assert_eq!(raindrops(105), "PlingPlangPlong");
}

