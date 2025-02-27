use crate::prelude::*;

/// A word in the word table (dereferences to `&str`)
///
/// First byte is the length (should never be more than 4),
/// and the rest is the static string value.
#[repr(transparent)]
pub struct Word {
	array: [u8; 5]
}

impl Word {
	#[expect(
		clippy::as_conversions,
		reason = "we enforce this usize to be small enough to fit in u8, that or u8 expand to usize"
	)]
	#[inline]
	const fn new(s: &[u8]) -> Self {
		assert!(s.len() <= 4);

		// I am screaming
		let aaaaaaaaaaaaa = b'A';
		let mut array = [aaaaaaaaaaaaa; 5];
		array[0] = s.len() as u8;

		let mut i = 0;
		while i < s.len() {
			let char = s[i];
			assert!(char != b' ');
			array[i + 1] = char;
			i += 1;
		}

		Self { array }
	}

	#[expect(clippy::as_conversions, reason = "u8 to usize not lossy")]
	#[inline]
	const fn len(&self) -> usize {
		self.array[0] as _
	}

	// not marking unsafe because only we can make valid instances of this struct
	// and external to this module you can only get references via the exported static
	#[inline]
	pub fn as_str(&self) -> &str {
		// this cannot be more than 4
		let len = self.len();
		// SAFETY: we just got ptr to array of len 5
		let ptr = unsafe { self.array.as_ptr().add(1) };
		// SAFETY: same
		let slice = unsafe { slice::from_raw_parts(ptr, len) };
		// SAFETY: only ASCII chars
		unsafe { str::from_utf8_unchecked(slice) }
	}
}

impl Deref for Word {
	type Target = str;

	#[inline]
	fn deref(&self) -> &str {
		self.as_str()
	}
}

// SAFETY: used for readonly static data, is fine
unsafe impl Sync for Word {}

macro_rules! make_array {
	{ $($str:literal)* } => {
		[$(Word::new($str)),*]
	}
}

pub static WORD_TABLE: [Word; 2048] = make_array! {
	b"A"    b"ABE"  b"ACE"  b"ACT"  b"AD"   b"ADA"  b"ADD"  b"AGO"  b"AID"  b"AIM"  b"AIR"  b"ALL"  b"ALP"  b"AM"   b"AMY"  b"AN"
	b"ANA"  b"AND"  b"ANN"  b"ANT"  b"ANY"  b"APE"  b"APS"  b"APT"  b"ARC"  b"ARE"  b"ARK"  b"ARM"  b"ART"  b"AS"   b"ASH"  b"ASK"
	b"AT"   b"ATE"  b"AUG"  b"AUK"  b"AVE"  b"AWE"  b"AWK"  b"AWL"  b"AWN"  b"AX"   b"AYE"  b"BAD"  b"BAG"  b"BAH"  b"BAM"  b"BAN"
	b"BAR"  b"BAT"  b"BAY"  b"BE"   b"BED"  b"BEE"  b"BEG"  b"BEN"  b"BET"  b"BEY"  b"BIB"  b"BID"  b"BIG"  b"BIN"  b"BIT"  b"BOB"
	b"BOG"  b"BON"  b"BOO"  b"BOP"  b"BOW"  b"BOY"  b"BUB"  b"BUD"  b"BUG"  b"BUM"  b"BUN"  b"BUS"  b"BUT"  b"BUY"  b"BY"   b"BYE"
	b"CAB"  b"CAL"  b"CAM"  b"CAN"  b"CAP"  b"CAR"  b"CAT"  b"CAW"  b"COD"  b"COG"  b"COL"  b"CON"  b"COO"  b"COP"  b"COT"  b"COW"
	b"COY"  b"CRY"  b"CUB"  b"CUE"  b"CUP"  b"CUR"  b"CUT"  b"DAB"  b"DAD"  b"DAM"  b"DAN"  b"DAR"  b"DAY"  b"DEE"  b"DEL"  b"DEN"
	b"DES"  b"DEW"  b"DID"  b"DIE"  b"DIG"  b"DIN"  b"DIP"  b"DO"   b"DOE"  b"DOG"  b"DON"  b"DOT"  b"DOW"  b"DRY"  b"DUB"  b"DUD"
	b"DUE"  b"DUG"  b"DUN"  b"EAR"  b"EAT"  b"ED"   b"EEL"  b"EGG"  b"EGO"  b"ELI"  b"ELK"  b"ELM"  b"ELY"  b"EM"   b"END"  b"EST"
	b"ETC"  b"EVA"  b"EVE"  b"EWE"  b"EYE"  b"FAD"  b"FAN"  b"FAR"  b"FAT"  b"FAY"  b"FED"  b"FEE"  b"FEW"  b"FIB"  b"FIG"  b"FIN"
	b"FIR"  b"FIT"  b"FLO"  b"FLY"  b"FOE"  b"FOG"  b"FOR"  b"FRY"  b"FUM"  b"FUN"  b"FUR"  b"GAB"  b"GAD"  b"GAG"  b"GAL"  b"GAM"
	b"GAP"  b"GAS"  b"GAY"  b"GEE"  b"GEL"  b"GEM"  b"GET"  b"GIG"  b"GIL"  b"GIN"  b"GO"   b"GOT"  b"GUM"  b"GUN"  b"GUS"  b"GUT"
	b"GUY"  b"GYM"  b"GYP"  b"HA"   b"HAD"  b"HAL"  b"HAM"  b"HAN"  b"HAP"  b"HAS"  b"HAT"  b"HAW"  b"HAY"  b"HE"   b"HEM"  b"HEN"
	b"HER"  b"HEW"  b"HEY"  b"HI"   b"HID"  b"HIM"  b"HIP"  b"HIS"  b"HIT"  b"HO"   b"HOB"  b"HOC"  b"HOE"  b"HOG"  b"HOP"  b"HOT"
	b"HOW"  b"HUB"  b"HUE"  b"HUG"  b"HUH"  b"HUM"  b"HUT"  b"I"    b"ICY"  b"IDA"  b"IF"   b"IKE"  b"ILL"  b"INK"  b"INN"  b"IO"
	b"ION"  b"IQ"   b"IRA"  b"IRE"  b"IRK"  b"IS"   b"IT"   b"ITS"  b"IVY"  b"JAB"  b"JAG"  b"JAM"  b"JAN"  b"JAR"  b"JAW"  b"JAY"
	b"JET"  b"JIG"  b"JIM"  b"JO"   b"JOB"  b"JOE"  b"JOG"  b"JOT"  b"JOY"  b"JUG"  b"JUT"  b"KAY"  b"KEG"  b"KEN"  b"KEY"  b"KID"
	b"KIM"  b"KIN"  b"KIT"  b"LA"   b"LAB"  b"LAC"  b"LAD"  b"LAG"  b"LAM"  b"LAP"  b"LAW"  b"LAY"  b"LEA"  b"LED"  b"LEE"  b"LEG"
	b"LEN"  b"LEO"  b"LET"  b"LEW"  b"LID"  b"LIE"  b"LIN"  b"LIP"  b"LIT"  b"LO"   b"LOB"  b"LOG"  b"LOP"  b"LOS"  b"LOT"  b"LOU"
	b"LOW"  b"LOY"  b"LUG"  b"LYE"  b"MA"   b"MAC"  b"MAD"  b"MAE"  b"MAN"  b"MAO"  b"MAP"  b"MAT"  b"MAW"  b"MAY"  b"ME"   b"MEG"
	b"MEL"  b"MEN"  b"MET"  b"MEW"  b"MID"  b"MIN"  b"MIT"  b"MOB"  b"MOD"  b"MOE"  b"MOO"  b"MOP"  b"MOS"  b"MOT"  b"MOW"  b"MUD"
	b"MUG"  b"MUM"  b"MY"   b"NAB"  b"NAG"  b"NAN"  b"NAP"  b"NAT"  b"NAY"  b"NE"   b"NED"  b"NEE"  b"NET"  b"NEW"  b"NIB"  b"NIL"
	b"NIP"  b"NIT"  b"NO"   b"NOB"  b"NOD"  b"NON"  b"NOR"  b"NOT"  b"NOV"  b"NOW"  b"NU"   b"NUN"  b"NUT"  b"O"    b"OAF"  b"OAK"
	b"OAR"  b"OAT"  b"ODD"  b"ODE"  b"OF"   b"OFF"  b"OFT"  b"OH"   b"OIL"  b"OK"   b"OLD"  b"ON"   b"ONE"  b"OR"   b"ORB"  b"ORE"
	b"ORR"  b"OS"   b"OTT"  b"OUR"  b"OUT"  b"OVA"  b"OW"   b"OWE"  b"OWL"  b"OWN"  b"OX"   b"PA"   b"PAD"  b"PAL"  b"PAM"  b"PAN"
	b"PAP"  b"PAR"  b"PAT"  b"PAW"  b"PAY"  b"PEA"  b"PEG"  b"PEN"  b"PEP"  b"PER"  b"PET"  b"PEW"  b"PHI"  b"PI"   b"PIE"  b"PIN"
	b"PIT"  b"PLY"  b"PO"   b"POD"  b"POE"  b"POP"  b"POT"  b"POW"  b"PRO"  b"PRY"  b"PUB"  b"PUG"  b"PUN"  b"PUP"  b"PUT"  b"QUO"
	b"RAG"  b"RAM"  b"RAN"  b"RAP"  b"RAT"  b"RAW"  b"RAY"  b"REB"  b"RED"  b"REP"  b"RET"  b"RIB"  b"RID"  b"RIG"  b"RIM"  b"RIO"
	b"RIP"  b"ROB"  b"ROD"  b"ROE"  b"RON"  b"ROT"  b"ROW"  b"ROY"  b"RUB"  b"RUE"  b"RUG"  b"RUM"  b"RUN"  b"RYE"  b"SAC"  b"SAD"
	b"SAG"  b"SAL"  b"SAM"  b"SAN"  b"SAP"  b"SAT"  b"SAW"  b"SAY"  b"SEA"  b"SEC"  b"SEE"  b"SEN"  b"SET"  b"SEW"  b"SHE"  b"SHY"
	b"SIN"  b"SIP"  b"SIR"  b"SIS"  b"SIT"  b"SKI"  b"SKY"  b"SLY"  b"SO"   b"SOB"  b"SOD"  b"SON"  b"SOP"  b"SOW"  b"SOY"  b"SPA"
	b"SPY"  b"SUB"  b"SUD"  b"SUE"  b"SUM"  b"SUN"  b"SUP"  b"TAB"  b"TAD"  b"TAG"  b"TAN"  b"TAP"  b"TAR"  b"TEA"  b"TED"  b"TEE"
	b"TEN"  b"THE"  b"THY"  b"TIC"  b"TIE"  b"TIM"  b"TIN"  b"TIP"  b"TO"   b"TOE"  b"TOG"  b"TOM"  b"TON"  b"TOO"  b"TOP"  b"TOW"
	b"TOY"  b"TRY"  b"TUB"  b"TUG"  b"TUM"  b"TUN"  b"TWO"  b"UN"   b"UP"   b"US"   b"USE"  b"VAN"  b"VAT"  b"VET"  b"VIE"  b"WAD"
	b"WAG"  b"WAR"  b"WAS"  b"WAY"  b"WE"   b"WEB"  b"WED"  b"WEE"  b"WET"  b"WHO"  b"WHY"  b"WIN"  b"WIT"  b"WOK"  b"WON"  b"WOO"
	b"WOW"  b"WRY"  b"WU"   b"YAM"  b"YAP"  b"YAW"  b"YE"   b"YEA"  b"YES"  b"YET"  b"YOU"  b"ABED" b"ABEL" b"ABET" b"ABLE" b"ABUT"
	b"ACHE" b"ACID" b"ACME" b"ACRE" b"ACTA" b"ACTS" b"ADAM" b"ADDS" b"ADEN" b"AFAR" b"AFRO" b"AGEE" b"AHEM" b"AHOY" b"AIDA" b"AIDE"
	b"AIDS" b"AIRY" b"AJAR" b"AKIN" b"ALAN" b"ALEC" b"ALGA" b"ALIA" b"ALLY" b"ALMA" b"ALOE" b"ALSO" b"ALTO" b"ALUM" b"ALVA" b"AMEN"
	b"AMES" b"AMID" b"AMMO" b"AMOK" b"AMOS" b"AMRA" b"ANDY" b"ANEW" b"ANNA" b"ANNE" b"ANTE" b"ANTI" b"AQUA" b"ARAB" b"ARCH" b"AREA"
	b"ARGO" b"ARID" b"ARMY" b"ARTS" b"ARTY" b"ASIA" b"ASKS" b"ATOM" b"AUNT" b"AURA" b"AUTO" b"AVER" b"AVID" b"AVIS" b"AVON" b"AVOW"
	b"AWAY" b"AWRY" b"BABE" b"BABY" b"BACH" b"BACK" b"BADE" b"BAIL" b"BAIT" b"BAKE" b"BALD" b"BALE" b"BALI" b"BALK" b"BALL" b"BALM"
	b"BAND" b"BANE" b"BANG" b"BANK" b"BARB" b"BARD" b"BARE" b"BARK" b"BARN" b"BARR" b"BASE" b"BASH" b"BASK" b"BASS" b"BATE" b"BATH"
	b"BAWD" b"BAWL" b"BEAD" b"BEAK" b"BEAM" b"BEAN" b"BEAR" b"BEAT" b"BEAU" b"BECK" b"BEEF" b"BEEN" b"BEER" b"BEET" b"BELA" b"BELL"
	b"BELT" b"BEND" b"BENT" b"BERG" b"BERN" b"BERT" b"BESS" b"BEST" b"BETA" b"BETH" b"BHOY" b"BIAS" b"BIDE" b"BIEN" b"BILE" b"BILK"
	b"BILL" b"BIND" b"BING" b"BIRD" b"BITE" b"BITS" b"BLAB" b"BLAT" b"BLED" b"BLEW" b"BLOB" b"BLOC" b"BLOT" b"BLOW" b"BLUE" b"BLUM"
	b"BLUR" b"BOAR" b"BOAT" b"BOCA" b"BOCK" b"BODE" b"BODY" b"BOGY" b"BOHR" b"BOIL" b"BOLD" b"BOLO" b"BOLT" b"BOMB" b"BONA" b"BOND"
	b"BONE" b"BONG" b"BONN" b"BONY" b"BOOK" b"BOOM" b"BOON" b"BOOT" b"BORE" b"BORG" b"BORN" b"BOSE" b"BOSS" b"BOTH" b"BOUT" b"BOWL"
	b"BOYD" b"BRAD" b"BRAE" b"BRAG" b"BRAN" b"BRAY" b"BRED" b"BREW" b"BRIG" b"BRIM" b"BROW" b"BUCK" b"BUDD" b"BUFF" b"BULB" b"BULK"
	b"BULL" b"BUNK" b"BUNT" b"BUOY" b"BURG" b"BURL" b"BURN" b"BURR" b"BURT" b"BURY" b"BUSH" b"BUSS" b"BUST" b"BUSY" b"BYTE" b"CADY"
	b"CAFE" b"CAGE" b"CAIN" b"CAKE" b"CALF" b"CALL" b"CALM" b"CAME" b"CANE" b"CANT" b"CARD" b"CARE" b"CARL" b"CARR" b"CART" b"CASE"
	b"CASH" b"CASK" b"CAST" b"CAVE" b"CEIL" b"CELL" b"CENT" b"CERN" b"CHAD" b"CHAR" b"CHAT" b"CHAW" b"CHEF" b"CHEN" b"CHEW" b"CHIC"
	b"CHIN" b"CHOU" b"CHOW" b"CHUB" b"CHUG" b"CHUM" b"CITE" b"CITY" b"CLAD" b"CLAM" b"CLAN" b"CLAW" b"CLAY" b"CLOD" b"CLOG" b"CLOT"
	b"CLUB" b"CLUE" b"COAL" b"COAT" b"COCA" b"COCK" b"COCO" b"CODA" b"CODE" b"CODY" b"COED" b"COIL" b"COIN" b"COKE" b"COLA" b"COLD"
	b"COLT" b"COMA" b"COMB" b"COME" b"COOK" b"COOL" b"COON" b"COOT" b"CORD" b"CORE" b"CORK" b"CORN" b"COST" b"COVE" b"COWL" b"CRAB"
	b"CRAG" b"CRAM" b"CRAY" b"CREW" b"CRIB" b"CROW" b"CRUD" b"CUBA" b"CUBE" b"CUFF" b"CULL" b"CULT" b"CUNY" b"CURB" b"CURD" b"CURE"
	b"CURL" b"CURT" b"CUTS" b"DADE" b"DALE" b"DAME" b"DANA" b"DANE" b"DANG" b"DANK" b"DARE" b"DARK" b"DARN" b"DART" b"DASH" b"DATA"
	b"DATE" b"DAVE" b"DAVY" b"DAWN" b"DAYS" b"DEAD" b"DEAF" b"DEAL" b"DEAN" b"DEAR" b"DEBT" b"DECK" b"DEED" b"DEEM" b"DEER" b"DEFT"
	b"DEFY" b"DELL" b"DENT" b"DENY" b"DESK" b"DIAL" b"DICE" b"DIED" b"DIET" b"DIME" b"DINE" b"DING" b"DINT" b"DIRE" b"DIRT" b"DISC"
	b"DISH" b"DISK" b"DIVE" b"DOCK" b"DOES" b"DOLE" b"DOLL" b"DOLT" b"DOME" b"DONE" b"DOOM" b"DOOR" b"DORA" b"DOSE" b"DOTE" b"DOUG"
	b"DOUR" b"DOVE" b"DOWN" b"DRAB" b"DRAG" b"DRAM" b"DRAW" b"DREW" b"DRUB" b"DRUG" b"DRUM" b"DUAL" b"DUCK" b"DUCT" b"DUEL" b"DUET"
	b"DUKE" b"DULL" b"DUMB" b"DUNE" b"DUNK" b"DUSK" b"DUST" b"DUTY" b"EACH" b"EARL" b"EARN" b"EASE" b"EAST" b"EASY" b"EBEN" b"ECHO"
	b"EDDY" b"EDEN" b"EDGE" b"EDGY" b"EDIT" b"EDNA" b"EGAN" b"ELAN" b"ELBA" b"ELLA" b"ELSE" b"EMIL" b"EMIT" b"EMMA" b"ENDS" b"ERIC"
	b"EROS" b"EVEN" b"EVER" b"EVIL" b"EYED" b"FACE" b"FACT" b"FADE" b"FAIL" b"FAIN" b"FAIR" b"FAKE" b"FALL" b"FAME" b"FANG" b"FARM"
	b"FAST" b"FATE" b"FAWN" b"FEAR" b"FEAT" b"FEED" b"FEEL" b"FEET" b"FELL" b"FELT" b"FEND" b"FERN" b"FEST" b"FEUD" b"FIEF" b"FIGS"
	b"FILE" b"FILL" b"FILM" b"FIND" b"FINE" b"FINK" b"FIRE" b"FIRM" b"FISH" b"FISK" b"FIST" b"FITS" b"FIVE" b"FLAG" b"FLAK" b"FLAM"
	b"FLAT" b"FLAW" b"FLEA" b"FLED" b"FLEW" b"FLIT" b"FLOC" b"FLOG" b"FLOW" b"FLUB" b"FLUE" b"FOAL" b"FOAM" b"FOGY" b"FOIL" b"FOLD"
	b"FOLK" b"FOND" b"FONT" b"FOOD" b"FOOL" b"FOOT" b"FORD" b"FORE" b"FORK" b"FORM" b"FORT" b"FOSS" b"FOUL" b"FOUR" b"FOWL" b"FRAU"
	b"FRAY" b"FRED" b"FREE" b"FRET" b"FREY" b"FROG" b"FROM" b"FUEL" b"FULL" b"FUME" b"FUND" b"FUNK" b"FURY" b"FUSE" b"FUSS" b"GAFF"
	b"GAGE" b"GAIL" b"GAIN" b"GAIT" b"GALA" b"GALE" b"GALL" b"GALT" b"GAME" b"GANG" b"GARB" b"GARY" b"GASH" b"GATE" b"GAUL" b"GAUR"
	b"GAVE" b"GAWK" b"GEAR" b"GELD" b"GENE" b"GENT" b"GERM" b"GETS" b"GIBE" b"GIFT" b"GILD" b"GILL" b"GILT" b"GINA" b"GIRD" b"GIRL"
	b"GIST" b"GIVE" b"GLAD" b"GLEE" b"GLEN" b"GLIB" b"GLOB" b"GLOM" b"GLOW" b"GLUE" b"GLUM" b"GLUT" b"GOAD" b"GOAL" b"GOAT" b"GOER"
	b"GOES" b"GOLD" b"GOLF" b"GONE" b"GONG" b"GOOD" b"GOOF" b"GORE" b"GORY" b"GOSH" b"GOUT" b"GOWN" b"GRAB" b"GRAD" b"GRAY" b"GREG"
	b"GREW" b"GREY" b"GRID" b"GRIM" b"GRIN" b"GRIT" b"GROW" b"GRUB" b"GULF" b"GULL" b"GUNK" b"GURU" b"GUSH" b"GUST" b"GWEN" b"GWYN"
	b"HAAG" b"HAAS" b"HACK" b"HAIL" b"HAIR" b"HALE" b"HALF" b"HALL" b"HALO" b"HALT" b"HAND" b"HANG" b"HANK" b"HANS" b"HARD" b"HARK"
	b"HARM" b"HART" b"HASH" b"HAST" b"HATE" b"HATH" b"HAUL" b"HAVE" b"HAWK" b"HAYS" b"HEAD" b"HEAL" b"HEAR" b"HEAT" b"HEBE" b"HECK"
	b"HEED" b"HEEL" b"HEFT" b"HELD" b"HELL" b"HELM" b"HERB" b"HERD" b"HERE" b"HERO" b"HERS" b"HESS" b"HEWN" b"HICK" b"HIDE" b"HIGH"
	b"HIKE" b"HILL" b"HILT" b"HIND" b"HINT" b"HIRE" b"HISS" b"HIVE" b"HOBO" b"HOCK" b"HOFF" b"HOLD" b"HOLE" b"HOLM" b"HOLT" b"HOME"
	b"HONE" b"HONK" b"HOOD" b"HOOF" b"HOOK" b"HOOT" b"HORN" b"HOSE" b"HOST" b"HOUR" b"HOVE" b"HOWE" b"HOWL" b"HOYT" b"HUCK" b"HUED"
	b"HUFF" b"HUGE" b"HUGH" b"HUGO" b"HULK" b"HULL" b"HUNK" b"HUNT" b"HURD" b"HURL" b"HURT" b"HUSH" b"HYDE" b"HYMN" b"IBIS" b"ICON"
	b"IDEA" b"IDLE" b"IFFY" b"INCA" b"INCH" b"INTO" b"IONS" b"IOTA" b"IOWA" b"IRIS" b"IRMA" b"IRON" b"ISLE" b"ITCH" b"ITEM" b"IVAN"
	b"JACK" b"JADE" b"JAIL" b"JAKE" b"JANE" b"JAVA" b"JEAN" b"JEFF" b"JERK" b"JESS" b"JEST" b"JIBE" b"JILL" b"JILT" b"JIVE" b"JOAN"
	b"JOBS" b"JOCK" b"JOEL" b"JOEY" b"JOHN" b"JOIN" b"JOKE" b"JOLT" b"JOVE" b"JUDD" b"JUDE" b"JUDO" b"JUDY" b"JUJU" b"JUKE" b"JULY"
	b"JUNE" b"JUNK" b"JUNO" b"JURY" b"JUST" b"JUTE" b"KAHN" b"KALE" b"KANE" b"KANT" b"KARL" b"KATE" b"KEEL" b"KEEN" b"KENO" b"KENT"
	b"KERN" b"KERR" b"KEYS" b"KICK" b"KILL" b"KIND" b"KING" b"KIRK" b"KISS" b"KITE" b"KLAN" b"KNEE" b"KNEW" b"KNIT" b"KNOB" b"KNOT"
	b"KNOW" b"KOCH" b"KONG" b"KUDO" b"KURD" b"KURT" b"KYLE" b"LACE" b"LACK" b"LACY" b"LADY" b"LAID" b"LAIN" b"LAIR" b"LAKE" b"LAMB"
	b"LAME" b"LAND" b"LANE" b"LANG" b"LARD" b"LARK" b"LASS" b"LAST" b"LATE" b"LAUD" b"LAVA" b"LAWN" b"LAWS" b"LAYS" b"LEAD" b"LEAF"
	b"LEAK" b"LEAN" b"LEAR" b"LEEK" b"LEER" b"LEFT" b"LEND" b"LENS" b"LENT" b"LEON" b"LESK" b"LESS" b"LEST" b"LETS" b"LIAR" b"LICE"
	b"LICK" b"LIED" b"LIEN" b"LIES" b"LIEU" b"LIFE" b"LIFT" b"LIKE" b"LILA" b"LILT" b"LILY" b"LIMA" b"LIMB" b"LIME" b"LIND" b"LINE"
	b"LINK" b"LINT" b"LION" b"LISA" b"LIST" b"LIVE" b"LOAD" b"LOAF" b"LOAM" b"LOAN" b"LOCK" b"LOFT" b"LOGE" b"LOIS" b"LOLA" b"LONE"
	b"LONG" b"LOOK" b"LOON" b"LOOT" b"LORD" b"LORE" b"LOSE" b"LOSS" b"LOST" b"LOUD" b"LOVE" b"LOWE" b"LUCK" b"LUCY" b"LUGE" b"LUKE"
	b"LULU" b"LUND" b"LUNG" b"LURA" b"LURE" b"LURK" b"LUSH" b"LUST" b"LYLE" b"LYNN" b"LYON" b"LYRA" b"MACE" b"MADE" b"MAGI" b"MAID"
	b"MAIL" b"MAIN" b"MAKE" b"MALE" b"MALI" b"MALL" b"MALT" b"MANA" b"MANN" b"MANY" b"MARC" b"MARE" b"MARK" b"MARS" b"MART" b"MARY"
	b"MASH" b"MASK" b"MASS" b"MAST" b"MATE" b"MATH" b"MAUL" b"MAYO" b"MEAD" b"MEAL" b"MEAN" b"MEAT" b"MEEK" b"MEET" b"MELD" b"MELT"
	b"MEMO" b"MEND" b"MENU" b"MERT" b"MESH" b"MESS" b"MICE" b"MIKE" b"MILD" b"MILE" b"MILK" b"MILL" b"MILT" b"MIMI" b"MIND" b"MINE"
	b"MINI" b"MINK" b"MINT" b"MIRE" b"MISS" b"MIST" b"MITE" b"MITT" b"MOAN" b"MOAT" b"MOCK" b"MODE" b"MOLD" b"MOLE" b"MOLL" b"MOLT"
	b"MONA" b"MONK" b"MONT" b"MOOD" b"MOON" b"MOOR" b"MOOT" b"MORE" b"MORN" b"MORT" b"MOSS" b"MOST" b"MOTH" b"MOVE" b"MUCH" b"MUCK"
	b"MUDD" b"MUFF" b"MULE" b"MULL" b"MURK" b"MUSH" b"MUST" b"MUTE" b"MUTT" b"MYRA" b"MYTH" b"NAGY" b"NAIL" b"NAIR" b"NAME" b"NARY"
	b"NASH" b"NAVE" b"NAVY" b"NEAL" b"NEAR" b"NEAT" b"NECK" b"NEED" b"NEIL" b"NELL" b"NEON" b"NERO" b"NESS" b"NEST" b"NEWS" b"NEWT"
	b"NIBS" b"NICE" b"NICK" b"NILE" b"NINA" b"NINE" b"NOAH" b"NODE" b"NOEL" b"NOLL" b"NONE" b"NOOK" b"NOON" b"NORM" b"NOSE" b"NOTE"
	b"NOUN" b"NOVA" b"NUDE" b"NULL" b"NUMB" b"OATH" b"OBEY" b"OBOE" b"ODIN" b"OHIO" b"OILY" b"OINT" b"OKAY" b"OLAF" b"OLDY" b"OLGA"
	b"OLIN" b"OMAN" b"OMEN" b"OMIT" b"ONCE" b"ONES" b"ONLY" b"ONTO" b"ONUS" b"ORAL" b"ORGY" b"OSLO" b"OTIS" b"OTTO" b"OUCH" b"OUST"
	b"OUTS" b"OVAL" b"OVEN" b"OVER" b"OWLY" b"OWNS" b"QUAD" b"QUIT" b"QUOD" b"RACE" b"RACK" b"RACY" b"RAFT" b"RAGE" b"RAID" b"RAIL"
	b"RAIN" b"RAKE" b"RANK" b"RANT" b"RARE" b"RASH" b"RATE" b"RAVE" b"RAYS" b"READ" b"REAL" b"REAM" b"REAR" b"RECK" b"REED" b"REEF"
	b"REEK" b"REEL" b"REID" b"REIN" b"RENA" b"REND" b"RENT" b"REST" b"RICE" b"RICH" b"RICK" b"RIDE" b"RIFT" b"RILL" b"RIME" b"RING"
	b"RINK" b"RISE" b"RISK" b"RITE" b"ROAD" b"ROAM" b"ROAR" b"ROBE" b"ROCK" b"RODE" b"ROIL" b"ROLL" b"ROME" b"ROOD" b"ROOF" b"ROOK"
	b"ROOM" b"ROOT" b"ROSA" b"ROSE" b"ROSS" b"ROSY" b"ROTH" b"ROUT" b"ROVE" b"ROWE" b"ROWS" b"RUBE" b"RUBY" b"RUDE" b"RUDY" b"RUIN"
	b"RULE" b"RUNG" b"RUNS" b"RUNT" b"RUSE" b"RUSH" b"RUSK" b"RUSS" b"RUST" b"RUTH" b"SACK" b"SAFE" b"SAGE" b"SAID" b"SAIL" b"SALE"
	b"SALK" b"SALT" b"SAME" b"SAND" b"SANE" b"SANG" b"SANK" b"SARA" b"SAUL" b"SAVE" b"SAYS" b"SCAN" b"SCAR" b"SCAT" b"SCOT" b"SEAL"
	b"SEAM" b"SEAR" b"SEAT" b"SEED" b"SEEK" b"SEEM" b"SEEN" b"SEES" b"SELF" b"SELL" b"SEND" b"SENT" b"SETS" b"SEWN" b"SHAG" b"SHAM"
	b"SHAW" b"SHAY" b"SHED" b"SHIM" b"SHIN" b"SHOD" b"SHOE" b"SHOT" b"SHOW" b"SHUN" b"SHUT" b"SICK" b"SIDE" b"SIFT" b"SIGH" b"SIGN"
	b"SILK" b"SILL" b"SILO" b"SILT" b"SINE" b"SING" b"SINK" b"SIRE" b"SITE" b"SITS" b"SITU" b"SKAT" b"SKEW" b"SKID" b"SKIM" b"SKIN"
	b"SKIT" b"SLAB" b"SLAM" b"SLAT" b"SLAY" b"SLED" b"SLEW" b"SLID" b"SLIM" b"SLIT" b"SLOB" b"SLOG" b"SLOT" b"SLOW" b"SLUG" b"SLUM"
	b"SLUR" b"SMOG" b"SMUG" b"SNAG" b"SNOB" b"SNOW" b"SNUB" b"SNUG" b"SOAK" b"SOAR" b"SOCK" b"SODA" b"SOFA" b"SOFT" b"SOIL" b"SOLD"
	b"SOME" b"SONG" b"SOON" b"SOOT" b"SORE" b"SORT" b"SOUL" b"SOUR" b"SOWN" b"STAB" b"STAG" b"STAN" b"STAR" b"STAY" b"STEM" b"STEW"
	b"STIR" b"STOW" b"STUB" b"STUN" b"SUCH" b"SUDS" b"SUIT" b"SULK" b"SUMS" b"SUNG" b"SUNK" b"SURE" b"SURF" b"SWAB" b"SWAG" b"SWAM"
	b"SWAN" b"SWAT" b"SWAY" b"SWIM" b"SWUM" b"TACK" b"TACT" b"TAIL" b"TAKE" b"TALE" b"TALK" b"TALL" b"TANK" b"TASK" b"TATE" b"TAUT"
	b"TEAL" b"TEAM" b"TEAR" b"TECH" b"TEEM" b"TEEN" b"TEET" b"TELL" b"TEND" b"TENT" b"TERM" b"TERN" b"TESS" b"TEST" b"THAN" b"THAT"
	b"THEE" b"THEM" b"THEN" b"THEY" b"THIN" b"THIS" b"THUD" b"THUG" b"TICK" b"TIDE" b"TIDY" b"TIED" b"TIER" b"TILE" b"TILL" b"TILT"
	b"TIME" b"TINA" b"TINE" b"TINT" b"TINY" b"TIRE" b"TOAD" b"TOGO" b"TOIL" b"TOLD" b"TOLL" b"TONE" b"TONG" b"TONY" b"TOOK" b"TOOL"
	b"TOOT" b"TORE" b"TORN" b"TOTE" b"TOUR" b"TOUT" b"TOWN" b"TRAG" b"TRAM" b"TRAY" b"TREE" b"TREK" b"TRIG" b"TRIM" b"TRIO" b"TROD"
	b"TROT" b"TROY" b"TRUE" b"TUBA" b"TUBE" b"TUCK" b"TUFT" b"TUNA" b"TUNE" b"TUNG" b"TURF" b"TURN" b"TUSK" b"TWIG" b"TWIN" b"TWIT"
	b"ULAN" b"UNIT" b"URGE" b"USED" b"USER" b"USES" b"UTAH" b"VAIL" b"VAIN" b"VALE" b"VARY" b"VASE" b"VAST" b"VEAL" b"VEDA" b"VEIL"
	b"VEIN" b"VEND" b"VENT" b"VERB" b"VERY" b"VETO" b"VICE" b"VIEW" b"VINE" b"VISE" b"VOID" b"VOLT" b"VOTE" b"WACK" b"WADE" b"WAGE"
	b"WAIL" b"WAIT" b"WAKE" b"WALE" b"WALK" b"WALL" b"WALT" b"WAND" b"WANE" b"WANG" b"WANT" b"WARD" b"WARM" b"WARN" b"WART" b"WASH"
	b"WAST" b"WATS" b"WATT" b"WAVE" b"WAVY" b"WAYS" b"WEAK" b"WEAL" b"WEAN" b"WEAR" b"WEED" b"WEEK" b"WEIR" b"WELD" b"WELL" b"WELT"
	b"WENT" b"WERE" b"WERT" b"WEST" b"WHAM" b"WHAT" b"WHEE" b"WHEN" b"WHET" b"WHOA" b"WHOM" b"WICK" b"WIFE" b"WILD" b"WILL" b"WIND"
	b"WINE" b"WING" b"WINK" b"WINO" b"WIRE" b"WISE" b"WISH" b"WITH" b"WOLF" b"WONT" b"WOOD" b"WOOL" b"WORD" b"WORE" b"WORK" b"WORM"
	b"WORN" b"WOVE" b"WRIT" b"WYNN" b"YALE" b"YANG" b"YANK" b"YARD" b"YARN" b"YAWL" b"YAWN" b"YEAH" b"YEAR" b"YELL" b"YOGA" b"YOKE"
};
