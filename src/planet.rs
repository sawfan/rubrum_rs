use crate::*;

use Planet::*;
use Sign::*;

// To be in rulership or exaltation is called being dignified, 
// and to be in detriment or fall is called being debilitated. 
// In traditional astrology both detriment and fall will tend to hinder 
// or create problems in the expression of that planet.
//
// A planet in a sign opposite to the one it rules is in detriment, 
// and a planet opposite its exaltation sign is said to be in fall. 


pub enum Planet {
  Pluto,
  Neptune,
  Uranus,

  Saturn,
  Jupiter,
  Mars,
  Sun,
  Venus,
  Mercury,
  Moon,
}

impl Planet {

  pub fn sign_rulerships(&self) -> Vec<Sign>{
    match self {
      Pluto   => vec![],
      Neptune => vec![],
      Uranus  => vec![],
      Saturn  => vec![Capricorn, Aquarius], 
      Jupiter => vec![Sagittarius, Pisces],
      Mars    => vec![Aries, Scorpio],
      Sun     => vec![Leo],
      Venus   => vec![Taurus, Libra],
      Mercury => vec![Gemini, Virgo],
      Moon    => vec![Cancer],
    }
  }

  // TODO: Look at NorthNode?
  // Exaltations have also been attributed to the north node 
  // (3rd degree of Gemini) and the south node (3rd degree of Sagittarius).
  //
  pub fn exaltation_sign(&self) -> Sign {
    match self {
      Pluto   => Scorpio,
      Neptune => Aquarius,
      Uranus  => Leo,
      Saturn  => Libra,
      Jupiter => Cancer,
      Mars    => Capricorn,
      Sun     => Aries,
      Venus   => Pisces,
      Mercury => Virgo,
      Moon    => Taurus,
      // NorthNode=> Gemini,
      // SouthNode=> Sagittarius,
    }
  }

  pub fn exaltation_degree(&self) -> u32 {
    match self {
      Pluto   => 3,
      Neptune => 19,
      Uranus  => 19,
      Saturn  => 21,
      Jupiter => 15,
      Mars    => 28,
      Sun     => 19,
      Venus   => 27,
      Mercury => 15,
      Moon    => 3,
      // NorthNode=> 3,
      // SouthNode=> 3,
    }
  }

  // TODO:
  pub fn fall(&self) -> Sign {
    match self {
      Pluto   => Taurus,
      Neptune => Leo,
      Uranus  => Aquarius,
      Saturn  => Aries,
      Jupiter => Capricorn,
      Mars    => Cancer,
      Sun     => Libra,
      Venus   => Virgo,
      Mercury => Pisces,
      Moon    => Scorpio,
      // NorthNode=> vec![],
      // SouthNode=> vec![],
    }
  }

  // TODO:
  pub fn detriment(&self) -> Vec<Sign> {
    match self {
      Pluto   => vec![],
      Neptune => vec![],
      Uranus  => vec![],
      Saturn  => vec![Cancer, Leo],
      Jupiter => vec![Gemini, Virgo],
      Mars    => vec![Libra, Taurus],
      Sun     => vec![Aquarius],
      Venus   => vec![Scorpio, Aries],
      Mercury => vec![Sagittarius, Pisces],
      Moon    => vec![Capricorn],
      // NorthNode=> vec![],
      // SouthNode=> vec![],
    }
  }



  pub fn emoji(&self) -> &str {
    planet_emoji::planet_emoji(self)
  }

  pub fn genethliac_value(&self) -> &str {
    match self {
      Pluto => "Will to Power", // Not from A.C.
      Neptune => "The True Self (Zeitgeist)",
      Uranus => "The True Will",
      Saturn => "The Ego (ahāṃkara)",
      Jupiter => "The Higher Love",
      Mars => "The Bodily Will",
      Sun => "The Human Will",
      Venus => "The Lower Love",
      Mercury => "The Mind",
      Moon => "The Senses",
    }
  }


  pub fn genethliac_bodily_value(&self) -> &str {
    match self {
      Pluto => "",
      Neptune => "Spiritual environment",
      Uranus => "Spiritual energy",
      Saturn => "Skeleton",
      Jupiter => "Wesenschau of Krause Self-realization through exploration of the inner world of the mind",
      Mars => "Muscular system",
      Sun => "Vital force. Spiritual conscious self",
      Venus => "",
      Mercury => "Cerebral tissues and nerves",
      Moon => "Bodily consciousness",
    }

  }

//  pub fn week_day(&self) -> WeekDay {
//    match self {
//      Pluto => panic!("Unknown metal for Pluto!"),
//      Neptune => panic!("Unknown metal for Neptune!"),
//      Uranus => panic!("Unknown metal for Uranus!"),
//      Saturn => WeekDay::Saturday,
//      Jupiter => WeekDay::Thursday,
//      Mars => WeekDay::Tuesday,
//      Sun => WeekDay::Sunday,
//      Venus => WeekDay::Friday,
//      Mercury => WeekDay::Wednesday,
//      Moon => WeekDay::Monday,
//    }
//  }


//  pub fn metal(&self) -> Metal {
//    match self {
//      Pluto => panic!("Unknown metal for Pluto!"),
//      Neptune => panic!("Unknown metal for Neptune!"),
//      Uranus => panic!("Unknown metal for Uranus!"),
//      Saturn => Metal::Lead,
//      Jupiter => Metal::Tin,
//      Mars => Metal::Iron,
//      Sun => Metal::Gold,
//      Venus => Metal::Copper,
//      Mercury => Metal::Mercury,
//      Moon => Metal::Silver,
//    }
//  }

 // pub fn humor(&self) -> Humor {
 //   match self {
 //     Pluto => panic!("Unknown humor for Pluto!"),
 //     Neptune => panic!("Unknown humor for Neptune!"),
 //     Uranus => panic!("Unknown humor for Uranus!"),
 //     Saturn => Humor::Melancholic,
 //     Jupiter => Humor::Sanguine,
 //     Mars => Humor::Choleric,
 //     Sun => Humor::Choleric,
 //     Venus => Humor::Phlematic,
 //     Mercury => panic!("Unknown humor for Mercury!"),
 //     Moon => Humor::Phlematic,

 //   }
 // }

 // pub fn humidity(&self) -> Humidity {
 //   match self {
 //     Pluto => panic!("Unknown humidity for Pluto!"),
 //     Neptune => panic!("Unknown humidity for Neptune!"),
 //     Uranus => panic!("Unknown humidity for Uranus!"),
 //     Saturn => Humidity::Dry,
 //     Jupiter => Humidity::Moist,
 //     Mars => Humidity::Dry,
 //     Sun => Humidity::Dry,
 //     Venus => Humidity::Moist,
 //     Mercury => panic!("Unknown humidity for Mercury!"),
 //     Moon => Humidity::Moist,

 //   }
 // }

 // pub fn temperature(&self) -> Temperature {
 //   match self {
 //     Pluto => panic!("Unknown temperature for Pluto!"),
 //     Neptune => panic!("Unknown temperature for Neptune!"),
 //     Uranus => panic!("Unknown temperature for Uranus!"),
 //     Saturn => Temperature::Cold,
 //     Jupiter => Temperature::Hot,
 //     Mars => Temperature::Hot,
 //     Sun => Temperature::Hot,
 //     Venus => Temperature::Cold,
 //     Mercury => panic!("Unknown temperature for Mercury!"),
 //     Moon => Temperature::Cold,

 //   }
 // }

}

