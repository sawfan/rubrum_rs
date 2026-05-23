use AndromedaKind::*;

pub const ALPHERATZ: AndromedaKind = AlphaAnd;
pub const SIRRAH: AndromedaKind = AlphaAnd;
pub const MIRACH: AndromedaKind = BetaAnd;
pub const ALMACH: AndromedaKind = GammaAnd;
pub const TITAWIN: AndromedaKind = UpsilonAnd;
pub const ADHIL: AndromedaKind = XiAnd;
pub const NEMBUS: AndromedaKind = _51And;
pub const HH_ANDROMEDAE: AndromedaKind = Ross248;
pub const VERITATE: AndromedaKind = _14And;

pub enum AndromedaKind {
    // is the brightest star in this constellation. It is an A0p class[10] binary star with an overall apparent visual magnitude of 2.1 and a luminosity of 96 L☉.[24] It is 97 light-years from Earth.[25] It represents Andromeda's head in Western mythology, however, the star's traditional Arabic names – Alpheratz and Sirrah, from the phrase surrat al-faras – [20] sometimes translated as "navel of the steed".[12][26][27] The Arabic names are a reference to the fact that α And forms an asterism known as the "Great Square of Pegasus" with 3 stars in Pegasus: α, β, and γ Peg. As such, the star was formerly considered to belong to both Andromeda and Pegasus, and was co-designated as "Delta Pegasi (δ Peg)", although this name is no longer formally used.[10][12][24]
    AlphaAnd,

    //is a red-hued giant star of type M0[10][28] located in an asterism known as the "girdle". It is 198 light-years away,[28] has a magnitude of 2.06,[29] and a luminosity of 115 L☉ with a planet discovered orbiting this star (b).[24] Its name comes from the Arabic phrase al-Maraqq meaning "the loins" or "the loincloth",[27] a phrase translated from Ptolemy's writing. However, β And was mostly considered by the Arabs to be a part of al-Hut, a constellation representing a larger fish than Pisces at Andromeda's feet.[20]
    BetaAnd,

    // is an orange-hued bright giant star of type K3[10] found at the southern tip of the constellation with an overall magnitude of 2.14.[24] Almach is a multiple star with a yellow primary of magnitude 2.3 and a blue-green secondary of magnitude 5, separated by 9.7 arcseconds.[11][12][26] British astronomer William Herschel said of the star: "[the] striking difference in the colour of the 2 stars, suggests the idea of a sun and its planet, to which the contrast of their unequal size contributes not a little."[30] The secondary, described by Herschel as a "fine light sky-blue, inclining to green",[30] is itself a double star, with a secondary of magnitude 6.3[11] and a period of 61 years.[24] The system is 358 light-years away.[31] Almach was named for the Arabic phrase ʿAnaq al-Ard, which means "the earth-kid", an obtuse reference to an animal that aids a lion in finding prey.[20][27]
    GammaAnd,

    // is an orange-hued giant star of type K3[10] orange giant of magnitude 3.3.[29] It is 105 light-years from Earth.[32]
    DeltaAnd,

    //ι And, κ, λ, ο, and ψ And form an asterism known as "Frederick's Glory", a name derived from a former constellation (Frederici Honores).[16] ι And is a blue-white hued main-sequence star of type B8, 502 light-years from Earth;[33] κ And is a white-hued main-sequence star of type B9 IVn, 168 light-years from Earth;[34] λ And is a yellow-hued giant star of type G8, 86 light-years from Earth;[35] ο And is a blue-white hued giant star of type B6, 679 light-years from Earth;[36] and ψ And is a blue-white hued main-sequence star of type B7, 988 light-years from Earth.[37]
    IotaAnd,

    //is a white-hued main-sequence star of type A5 and magnitude 3.9.[29] It is 130 light-years away.[38]
    MuAnd,

    // is a magnitude 4.1[29] binary system that consists of one F-type dwarf and an M-type dwarf. The primary star has a planetary system with 4 confirmed planets,[40] 0.96 times, 14.57 times, 10.19 times and 1.06 the mass of Jupiter.[41] The system is 44 light-years from Earth.[42]
    UpsilonAnd,

    //is a binary star 217 light-years away. The primary is an orange-hued giant star of type K0.[43]
    XiAnd,

    //is a blue-white hued binary star of magnitude 4.3[29] that is 598 light-years away. The primary is a main-sequence star of type B5.[44] Its companion star is of magnitude 8.9.[29]
    PiAnd,

    //was assigned by Johann Bayer to Perseus, where he designated it "Upsilon Persei (υ Per)", but it was moved to Andromeda by the International Astronomical Union.[45] It is 177 light-years from Earth and is an orange-hued giant star of type K3.[46]
    _51And,

    //was a former designation for φ Per.[12][45]
    _54And,

    // is an optical binary star. The primary is a yellow-hued giant star of type K0 with an apparent magnitude of 5.7[29] that is 316 light-years away.[47] The secondary is an orange-hued giant star of type K0 and magnitude 5.9 that is 990 light-years from Earth.[29]
    _56And,

    // is a Mira-type variable star with a period of 409 days. Its maximum magnitude is 5.8 and its minimum magnitude is 14.8,[10] and it is at a distance of 1,250 light-years.[48] There are 6 other Mira variables in Andromeda.[24]
    RAnd,

    // is the M-type prototype for its class of variable stars. It ranges in magnitude from a minimum of 12.4 to a maximum of 8.[24] It is 2,720 light-years away.[49]
    ZAnd,

    // is the ninth-closest star to Earth at a distance of 10.3 light-years.[7] It is a red-hued main-sequence BY Draconis variable star of type M6.[50]
    Ross248, //(HH Andromedae)

    // is a yellow-hued giant star of type G8 that is 251 light-years away.[51] It has a mass of 2.2 M☉ and a radius of 11 R☉. It has one planet, 14 Andromedae b, discovered in 2008. It orbits at a distance of 0.83 astronomical units from its parent star every 186 days and has a mass of 4.3 MJ.[52]
    _14And,
}
