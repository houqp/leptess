// ⚠️ This file is generated
// ⚠️ Regenerate with `make src/variable.rs`

use std::ffi::CStr;

/// Enum representing different variable options accepted by Tesseract
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Variable {
    /// Number of Class Pruner Levels
    ///
    /// Example value: `"3"`
    ClassifyNumCpLevels,
    /// Max pixel gap for broken pixed pitch
    ///
    /// Example value: `"3"`
    TextordDotmatrixGap,
    /// Block to do debug on
    ///
    /// Example value: `"0"`
    TextordDebugBlock,
    /// Max range test on pitch
    ///
    /// Example value: `"2"`
    TextordPitchRange,
    /// Rows required to outvote a veto
    ///
    /// Example value: `"5"`
    TextordWordsVetoPower,
    /// Show stroke widths
    ///
    /// Example value: `"0"`
    TextordTabfindShowStrokewidths,
    /// Use new fast algorithm
    ///
    /// Example value: `"6"`
    PitsyncLinearVersion,
    /// Max advance fake generation
    ///
    /// Example value: `"1"`
    PitsyncFakeDepth,
    /// Max lost before fallback line used
    ///
    /// Example value: `"10"`
    OldblHoledLosscount,
    /// For smooth factor
    ///
    /// Example value: `"4"`
    TextordSkewsmoothOffset,
    /// For smooth factor
    ///
    /// Example value: `"1"`
    TextordSkewsmoothOffset2,
    /// coord of test pt
    ///
    /// Example value: `"-2147483647"`
    TextordTestX,
    /// coord of test pt
    ///
    /// Example value: `"-2147483647"`
    TextordTestY,
    /// Min blobs before gradient counted
    ///
    /// Example value: `"4"`
    TextordMinBlobsInRow,
    /// Min blobs in each spline segment
    ///
    /// Example value: `"8"`
    TextordSplineMinblobs,
    /// Size of window for spline segmentation
    ///
    /// Example value: `"6"`
    TextordSplineMedianwin,
    /// Max number of blobs a big blob can overlap
    ///
    /// Example value: `"4"`
    TextordMaxBlobOverlaps,
    /// Min credible pixel xheight
    ///
    /// Example value: `"10"`
    TextordMinXheight,
    /// Number of linew fits to do
    ///
    /// Example value: `"12"`
    TextordLmsLineTrials,
    /// Show image blobs
    ///
    /// Example value: `"0"`
    TextordTabfindShowImages,
    /// Max allowed bending of chop cells
    ///
    /// Example value: `"2"`
    TextordFpChopError,
    /// Max number of children inside a character outline
    ///
    /// Example value: `"10"`
    EdgesMaxChildrenPerOutline,
    /// Max layers of nested children inside a character outline
    ///
    /// Example value: `"5"`
    EdgesMaxChildrenLayers,
    /// Importance ratio for chucking outlines
    ///
    /// Example value: `"10"`
    EdgesChildrenPerGrandchild,
    /// Max holes allowed in blob
    ///
    /// Example value: `"45"`
    EdgesChildrenCountLimit,
    /// Min pixels for potential char in box
    ///
    /// Example value: `"12"`
    EdgesMinNonhole,
    /// Max lensq/area for acceptable child outline
    ///
    /// Example value: `"40"`
    EdgesPathareaRatio,
    /// Debug level for split shiro\-rekha process\.
    ///
    /// Example value: `"0"`
    DevanagariSplitDebuglevel,
    /// Show partition bounds, waiting if >1
    ///
    /// Example value: `"0"`
    TextordTabfindShowPartitions,
    /// Debug tab finding
    ///
    /// Example value: `"0"`
    TextordDebugTabfind,
    /// Turn on output related to bugs in tab finding
    ///
    /// Example value: `"0"`
    TextordDebugBugs,
    /// Left edge of debug reporting rectangle
    ///
    /// Example value: `"-1"`
    TextordTestregionLeft,
    /// Top edge of debug reporting rectangle
    ///
    /// Example value: `"-1"`
    TextordTestregionTop,
    /// Right edge of debug rectangle
    ///
    /// Example value: `"2147483647"`
    TextordTestregionRight,
    /// Bottom edge of debug rectangle
    ///
    /// Example value: `"2147483647"`
    TextordTestregionBottom,
    /// Editor image X Pos
    ///
    /// Example value: `"590"`
    EditorImageXpos,
    /// Editor image Y Pos
    ///
    /// Example value: `"10"`
    EditorImageYpos,
    /// Add to image height for menu bar
    ///
    /// Example value: `"50"`
    EditorImageMenuheight,
    /// Word bounding box colour
    ///
    /// Example value: `"7"`
    EditorImageWordBbColor,
    /// Blob bounding box colour
    ///
    /// Example value: `"4"`
    EditorImageBlobBbColor,
    /// Correct text colour
    ///
    /// Example value: `"2"`
    EditorImageTextColor,
    /// Editor debug window X Pos
    ///
    /// Example value: `"50"`
    EditorDbwinXpos,
    /// Editor debug window Y Pos
    ///
    /// Example value: `"500"`
    EditorDbwinYpos,
    /// Editor debug window height
    ///
    /// Example value: `"24"`
    EditorDbwinHeight,
    /// Editor debug window width
    ///
    /// Example value: `"80"`
    EditorDbwinWidth,
    /// Word window X Pos
    ///
    /// Example value: `"60"`
    EditorWordXpos,
    /// Word window Y Pos
    ///
    /// Example value: `"510"`
    EditorWordYpos,
    /// Word window height
    ///
    /// Example value: `"240"`
    EditorWordHeight,
    /// Word window width
    ///
    /// Example value: `"655"`
    EditorWordWidth,
    /// Display splits
    ///
    /// Example value: `"0"`
    WordrecDisplaySplits,
    /// Debug old poly
    ///
    /// Example value: `"0"`
    PolyDebug,
    /// More accurate approx on wide things
    ///
    /// Example value: `"1"`
    PolyWideObjectsBetter,
    /// Display Blobs
    ///
    /// Example value: `"0"`
    WordrecDisplayAllBlobs,
    /// Blob pause
    ///
    /// Example value: `"0"`
    WordrecBlobPause,
    /// Do fixed pitch chopping
    ///
    /// Example value: `"1"`
    TextordFpChopping,
    /// Force proportional word segmentation on all rows
    ///
    /// Example value: `"0"`
    TextordForceMakePropWords,
    /// Chopper is being tested\.
    ///
    /// Example value: `"0"`
    TextordChopperTest,
    /// Chop underlines & put back
    ///
    /// Example value: `"1"`
    TextordRestoreUnderlines,
    /// Display separate words
    ///
    /// Example value: `"0"`
    TextordShowInitialWords,
    /// Display separate words
    ///
    /// Example value: `"0"`
    TextordShowNewWords,
    /// Display forced fixed pitch words
    ///
    /// Example value: `"0"`
    TextordShowFixedWords,
    /// Moan about prop blocks
    ///
    /// Example value: `"0"`
    TextordBlocksallFixed,
    /// Moan about fixed pitch blocks
    ///
    /// Example value: `"0"`
    TextordBlocksallProp,
    /// Dump stats when moaning
    ///
    /// Example value: `"0"`
    TextordBlocksallTesting,
    /// Do current test
    ///
    /// Example value: `"0"`
    TextordTestMode,
    /// Scale scores on big words
    ///
    /// Example value: `"0"`
    TextordPitchScalebigwords,
    /// All doc is proportial text
    ///
    /// Example value: `"0"`
    TextordAllProp,
    /// Debug on fixed pitch test
    ///
    /// Example value: `"0"`
    TextordDebugPitchTest,
    /// Turn off dp fixed pitch algorithm
    ///
    /// Example value: `"0"`
    TextordDisablePitchTest,
    /// Do even faster pitch algorithm
    ///
    /// Example value: `"0"`
    TextordFastPitchTest,
    /// Write full metric stuff
    ///
    /// Example value: `"0"`
    TextordDebugPitchMetric,
    /// Draw row\-level cuts
    ///
    /// Example value: `"0"`
    TextordShowRowCuts,
    /// Draw page\-level cuts
    ///
    /// Example value: `"0"`
    TextordShowPageCuts,
    /// Use correct answer for fixed/prop
    ///
    /// Example value: `"0"`
    TextordPitchCheat,
    /// Attempt whole doc/block fixed pitch
    ///
    /// Example value: `"0"`
    TextordBlockndocFixed,
    /// Show table regions
    ///
    /// Example value: `"0"`
    TextordShowTables,
    /// Debug table marking steps in detail
    ///
    /// Example value: `"0"`
    TextordTablefindShowMark,
    /// Show page stats used in table finding
    ///
    /// Example value: `"0"`
    TextordTablefindShowStats,
    /// Enables the table recognizer for table layout and filtering\.
    ///
    /// Example value: `"0"`
    TextordTablefindRecognizeTables,
    /// Show tab candidates
    ///
    /// Example value: `"0"`
    TextordTabfindShowInitialtabs,
    /// Show tab vectors
    ///
    /// Example value: `"0"`
    TextordTabfindShowFinaltabs,
    /// Only run stroke widths
    ///
    /// Example value: `"0"`
    TextordTabfindOnlyStrokewidths,
    /// Use original wiseowl xheight
    ///
    /// Example value: `"0"`
    TextordReallyOldXheight,
    /// Debug old baseline generation
    ///
    /// Example value: `"0"`
    TextordOldblDebug,
    /// Debug baseline generation
    ///
    /// Example value: `"0"`
    TextordDebugBaselines,
    /// Use para default mechanism
    ///
    /// Example value: `"1"`
    TextordOldblParadef,
    /// Split stepped splines
    ///
    /// Example value: `"1"`
    TextordOldblSplitSplines,
    /// Merge suspect partitions
    ///
    /// Example value: `"1"`
    TextordOldblMergeParts,
    /// Improve correlation of heights
    ///
    /// Example value: `"1"`
    OldblCorrfix,
    /// Fix bug in modes threshold for xheights
    ///
    /// Example value: `"0"`
    OldblXhfix,
    /// Make baselines for ocropus
    ///
    /// Example value: `"0"`
    TextordOcropusMode,
    /// Vigorously remove noise
    ///
    /// Example value: `"0"`
    TextordHeavyNr,
    /// Display row accumulation
    ///
    /// Example value: `"0"`
    TextordShowInitialRows,
    /// Display page correlated rows
    ///
    /// Example value: `"0"`
    TextordShowParallelRows,
    /// Display rows after expanding
    ///
    /// Example value: `"0"`
    TextordShowExpandedRows,
    /// Display rows after final fitting
    ///
    /// Example value: `"0"`
    TextordShowFinalRows,
    /// Display blob bounds after pre\-ass
    ///
    /// Example value: `"0"`
    TextordShowFinalBlobs,
    /// Tests refer to land/port
    ///
    /// Example value: `"0"`
    TextordTestLandscape,
    /// Force parallel baselines
    ///
    /// Example value: `"1"`
    TextordParallelBaselines,
    /// Force straight baselines
    ///
    /// Example value: `"0"`
    TextordStraightBaselines,
    /// Use old baseline algorithm
    ///
    /// Example value: `"1"`
    TextordOldBaselines,
    /// Use old xheight algorithm
    ///
    /// Example value: `"0"`
    TextordOldXheight,
    /// Use spline baseline
    ///
    /// Example value: `"1"`
    TextordFixXheightBug,
    /// Prevent multiple baselines
    ///
    /// Example value: `"1"`
    TextordFixMakerowBug,
    /// Test xheight algorithms
    ///
    /// Example value: `"0"`
    TextordDebugXheights,
    /// Bias skew estimates with line length
    ///
    /// Example value: `"1"`
    TextordBiasedSkewcalc,
    /// Interpolate across gaps
    ///
    /// Example value: `"1"`
    TextordInterpolatingSkew,
    /// Use test xheight mechanism
    ///
    /// Example value: `"1"`
    TextordNewInitialXheight,
    /// Print test blob information
    ///
    /// Example value: `"0"`
    TextordDebugBlob,
    /// Say which blocks have tables
    ///
    /// Example value: `"0"`
    GapmapDebug,
    /// Use large space at start and end of rows
    ///
    /// Example value: `"0"`
    GapmapUseEnds,
    /// Ensure gaps not less than 2quanta wide
    ///
    /// Example value: `"0"`
    GapmapNoIsolatedQuanta,
    /// Use the new outline complexity module
    ///
    /// Example value: `"0"`
    EdgesUseNewOutlineComplexity,
    /// turn on debugging for this module
    ///
    /// Example value: `"0"`
    EdgesDebug,
    /// Remove boxy parents of char\-like children
    ///
    /// Example value: `"0"`
    EdgesChildrenFix,
    /// Draw fixed pitch cell boundaries
    ///
    /// Example value: `"0"`
    TextordShowFixedCuts,
    /// Whether to create a debug image for split shiro\-rekha process\.
    ///
    /// Example value: `"0"`
    DevanagariSplitDebugimage,
    /// Show partition bounds
    ///
    /// Example value: `"0"`
    TextordTabfindShowInitialPartitions,
    /// Show blobs rejected as noise
    ///
    /// Example value: `"0"`
    TextordTabfindShowRejectBlobs,
    /// Show column bounds
    ///
    /// Example value: `"0"`
    TextordTabfindShowColumns,
    /// Show final block bounds
    ///
    /// Example value: `"0"`
    TextordTabfindShowBlocks,
    /// run table detection
    ///
    /// Example value: `"1"`
    TextordTabfindFindTables,
    /// If true, word delimiter spaces are assumed to have variable width, even though characters have fixed pitch\.
    ///
    /// Example value: `"0"`
    TextordSpaceSizeIsVariable,
    /// Make debug windows printable
    ///
    /// Example value: `"0"`
    TextordDebugPrintable,
    /// Save input bi image
    ///
    /// Example value: `"0"`
    EquationdetectSaveBiImage,
    /// Save special character image
    ///
    /// Example value: `"0"`
    EquationdetectSaveSptImage,
    /// Save the seed image
    ///
    /// Example value: `"0"`
    EquationdetectSaveSeedImage,
    /// Save the merged image
    ///
    /// Example value: `"0"`
    EquationdetectSaveMergedImage,
    /// Stream a filelist from stdin
    ///
    /// Example value: `"0"`
    StreamFilelist,
    /// File to send tprintf output to
    DebugFile,
    /// Function used for calculation of dot product
    ///
    /// Example value: `"generic"`
    Dotproduct,
    /// Default font name to be used in training
    ///
    /// Example value: `"UnknownFont"`
    ClassifyFontName,
    /// Name of debugfile
    ///
    /// Example value: `"FXDebug"`
    FxDebugfile,
    /// Editor image window name
    ///
    /// Example value: `"EditorImage"`
    EditorImageWinName,
    /// Editor debug window name
    ///
    /// Example value: `"EditorDBWin"`
    EditorDbwinName,
    /// BL normalized word window
    ///
    /// Example value: `"BlnWords"`
    EditorWordName,
    /// Title of output document \(used for hOCR and PDF output\)
    DocumentTitle,
    /// Pico Feature Length
    ///
    /// Example value: `"0.05"`
    ClassifyPicoFeatureLength,
    /// Norm adjust midpoint \.\.\.
    ///
    /// Example value: `"32"`
    ClassifyNormAdjMidpoint,
    /// Norm adjust curl \.\.\.
    ///
    /// Example value: `"2"`
    ClassifyNormAdjCurl,
    /// Slope below which lines are called horizontal
    ///
    /// Example value: `"0.414214"`
    ClassifyMinSlope,
    /// Slope above which lines are called vertical
    ///
    /// Example value: `"2.41421"`
    ClassifyMaxSlope,
    /// Class Pruner Angle Pad Loose
    ///
    /// Example value: `"45"`
    ClassifyCpAnglePadLoose,
    /// Class Pruner Angle Pad Medium
    ///
    /// Example value: `"20"`
    ClassifyCpAnglePadMedium,
    /// CLass Pruner Angle Pad Tight
    ///
    /// Example value: `"10"`
    ClassifyCpAnglePadTight,
    /// Class Pruner End Pad Loose
    ///
    /// Example value: `"0.5"`
    ClassifyCpEndPadLoose,
    /// Class Pruner End Pad Medium
    ///
    /// Example value: `"0.5"`
    ClassifyCpEndPadMedium,
    /// Class Pruner End Pad Tight
    ///
    /// Example value: `"0.5"`
    ClassifyCpEndPadTight,
    /// Class Pruner Side Pad Loose
    ///
    /// Example value: `"2.5"`
    ClassifyCpSidePadLoose,
    /// Class Pruner Side Pad Medium
    ///
    /// Example value: `"1.2"`
    ClassifyCpSidePadMedium,
    /// Class Pruner Side Pad Tight
    ///
    /// Example value: `"0.6"`
    ClassifyCpSidePadTight,
    /// Proto Pruner Angle Pad
    ///
    /// Example value: `"45"`
    ClassifyPpAnglePad,
    /// Proto Prune End Pad
    ///
    /// Example value: `"0.5"`
    ClassifyPpEndPad,
    /// Proto Pruner Side Pad
    ///
    /// Example value: `"2.5"`
    ClassifyPpSidePad,
    /// Fraction of x to ignore
    ///
    /// Example value: `"0.1"`
    TextordUnderlineOffset,
    /// Smoothing gap stats
    ///
    /// Example value: `"0.05"`
    TextordWordstatsSmoothFactor,
    /// Smoothing width stats
    ///
    /// Example value: `"0.1"`
    TextordWidthSmoothFactor,
    /// Ile of blob widths for space est
    ///
    /// Example value: `"0.4"`
    TextordWordsWidthIle,
    /// Multiple of xheight
    ///
    /// Example value: `"4"`
    TextordWordsMaxspace,
    /// Max believable third space
    ///
    /// Example value: `"3.5"`
    TextordWordsDefaultMaxspace,
    /// Fraction of xheight
    ///
    /// Example value: `"0.6"`
    TextordWordsDefaultMinspace,
    /// Fraction of xheight
    ///
    /// Example value: `"0.3"`
    TextordWordsMinMinspace,
    /// Fraction of xheight
    ///
    /// Example value: `"0.2"`
    TextordWordsDefaultNonspace,
    /// Max initial cluster size
    ///
    /// Example value: `"0.25"`
    TextordWordsInitialLower,
    /// Min initial cluster spacing
    ///
    /// Example value: `"0.15"`
    TextordWordsInitialUpper,
    /// Fraction of valid gaps needed
    ///
    /// Example value: `"0.75"`
    TextordWordsMinlarge,
    /// Pitch sync threshold
    ///
    /// Example value: `"0.04"`
    TextordWordsPitchsdThreshold,
    /// Threshold for definite fixed
    ///
    /// Example value: `"0.016"`
    TextordWordsDefFixed,
    /// Threshold for definite prop
    ///
    /// Example value: `"0.09"`
    TextordWordsDefProp,
    /// Fraction of xheight for sameness
    ///
    /// Example value: `"0.08"`
    TextordPitchRowsimilarity,
    /// Max initial cluster size
    ///
    /// Example value: `"0.5"`
    WordsInitialLower,
    /// Min initial cluster spacing
    ///
    /// Example value: `"0.15"`
    WordsInitialUpper,
    /// Fraction of xheight
    ///
    /// Example value: `"0.25"`
    WordsDefaultPropNonspace,
    /// Fraction of xheight
    ///
    /// Example value: `"0.75"`
    WordsDefaultFixedSpace,
    /// Allowed size variance
    ///
    /// Example value: `"0.6"`
    WordsDefaultFixedLimit,
    /// Non\-fuzzy spacing region
    ///
    /// Example value: `"0.3"`
    TextordWordsDefiniteSpread,
    /// Min ratio space/nonspace
    ///
    /// Example value: `"2.8"`
    TextordSpacesizeRatiofp,
    /// Min ratio space/nonspace
    ///
    /// Example value: `"2"`
    TextordSpacesizeRatioprop,
    /// Pitch IQR/Gap IQR threshold
    ///
    /// Example value: `"1.5"`
    TextordFpiqrRatio,
    /// Xh fraction noise in pitch
    ///
    /// Example value: `"0.2"`
    TextordMaxPitchIqr,
    /// Min width of decent blobs
    ///
    /// Example value: `"0.5"`
    TextordFpMinWidth,
    /// Ding rate for mid\-cuts
    ///
    /// Example value: `"0.2"`
    TextordProjectionScale,
    /// Ding rate for unbalanced char cells
    ///
    /// Example value: `"1"`
    TextordBalanceFactor,
    /// max fraction of mean blob width allowed for vertical gaps in vertical text
    ///
    /// Example value: `"0.5"`
    TextordTabvectorVerticalGapFraction,
    /// Fraction of box matches required to declare a line vertical
    ///
    /// Example value: `"0.5"`
    TextordTabvectorVerticalBoxRatio,
    /// Dist inside big blob for chopping
    ///
    /// Example value: `"0.75"`
    PitsyncJoinedEdge,
    /// Fraction of cut for free cuts
    ///
    /// Example value: `"0.25"`
    PitsyncOffsetFreecutFraction,
    /// Fraction of est allowed in calc
    ///
    /// Example value: `"0.4"`
    OldblXhfract,
    /// Max aspect ratio of a dot
    ///
    /// Example value: `"1.26"`
    OldblDotErrorSize,
    /// X fraction for new partition
    ///
    /// Example value: `"0.15"`
    TextordOldblJumplimit,
    /// Fraction of line spacing for quad
    ///
    /// Example value: `"0.02"`
    TextordSplineShiftFraction,
    /// Fraction of line spacing for outlier
    ///
    /// Example value: `"0.1"`
    TextordSplineOutlierFraction,
    /// Ile of gradients for page skew
    ///
    /// Example value: `"0.5"`
    TextordSkewIle,
    /// Lag for skew on row accumulation
    ///
    /// Example value: `"0.02"`
    TextordSkewLag,
    /// Max iqr/median for linespace
    ///
    /// Example value: `"0.2"`
    TextordLinespaceIqrlimit,
    /// Max width of blobs to make rows
    ///
    /// Example value: `"8"`
    TextordWidthLimit,
    /// Max width before chopping
    ///
    /// Example value: `"1.5"`
    TextordChopWidth,
    /// Factor to expand rows by in expand\_rows
    ///
    /// Example value: `"1"`
    TextordExpansionFactor,
    /// Fraction of linespace for good overlap
    ///
    /// Example value: `"0.375"`
    TextordOverlapX,
    /// fraction of linesize for min xheight
    ///
    /// Example value: `"0.25"`
    TextordMinxh,
    /// \* blob height for initial linesize
    ///
    /// Example value: `"1.25"`
    TextordMinLinesize,
    /// New row made if blob makes row this big
    ///
    /// Example value: `"1.3"`
    TextordExcessBlobsize,
    /// Fraction of neighbourhood
    ///
    /// Example value: `"0.4"`
    TextordOccupancyThreshold,
    /// Multiple of line\_size for underline
    ///
    /// Example value: `"2"`
    TextordUnderlineWidth,
    /// Min blob height/top to include blob top into xheight stats
    ///
    /// Example value: `"0.75"`
    TextordMinBlobHeightFraction,
    /// Min pile height to make xheight
    ///
    /// Example value: `"0.4"`
    TextordXheightModeFraction,
    /// Min pile height to make ascheight
    ///
    /// Example value: `"0.08"`
    TextordAscheightModeFraction,
    /// Min pile height to make descheight
    ///
    /// Example value: `"0.08"`
    TextordDescheightModeFraction,
    /// Min cap/xheight
    ///
    /// Example value: `"1.25"`
    TextordAscxRatioMin,
    /// Max cap/xheight
    ///
    /// Example value: `"1.8"`
    TextordAscxRatioMax,
    /// Min desc/xheight
    ///
    /// Example value: `"0.25"`
    TextordDescxRatioMin,
    /// Max desc/xheight
    ///
    /// Example value: `"0.6"`
    TextordDescxRatioMax,
    /// Accepted variation
    ///
    /// Example value: `"0.1"`
    TextordXheightErrorMargin,
    /// xht multiplier
    ///
    /// Example value: `"1.75"`
    GapmapBigGaps,
    /// Max distance of chop pt from vertex
    ///
    /// Example value: `"0.5"`
    TextordFpChopSnap,
    /// Min area fraction of child outline
    ///
    /// Example value: `"0.5"`
    EdgesChildarea,
    /// Min area fraction of grandchild for box
    ///
    /// Example value: `"0.875"`
    EdgesBoxarea,
    /// Fraction of width occupied
    ///
    /// Example value: `"0.5"`
    TextordUnderlineThreshold,
    /// Debug level for unichar ambiguities
    ///
    /// Example value: `"0"`
    AmbigsDebugLevel,
    /// Classify debug level
    ///
    /// Example value: `"0"`
    ClassifyDebugLevel,
    /// Normalization Method   \.\.\.
    ///
    /// Example value: `"1"`
    ClassifyNormMethod,
    /// Matcher Debug Level
    ///
    /// Example value: `"0"`
    MatcherDebugLevel,
    /// Matcher Debug Flags
    ///
    /// Example value: `"0"`
    MatcherDebugFlags,
    /// Learning Debug Level: 
    ///
    /// Example value: `"0"`
    ClassifyLearningDebugLevel,
    /// Min \# of permanent classes
    ///
    /// Example value: `"1"`
    MatcherPermanentClassesMin,
    /// Reliable Config Threshold
    ///
    /// Example value: `"3"`
    MatcherMinExamplesForPrototyping,
    /// Enable adaption even if the ambiguities have not been seen
    ///
    /// Example value: `"5"`
    MatcherSufficientExamplesForPrototyping,
    /// Threshold for good protos during adaptive 0\-255
    ///
    /// Example value: `"230"`
    ClassifyAdaptProtoThreshold,
    /// Threshold for good features during adaptive 0\-255
    ///
    /// Example value: `"230"`
    ClassifyAdaptFeatureThreshold,
    /// Class Pruner Threshold 0\-255
    ///
    /// Example value: `"229"`
    ClassifyClassPrunerThreshold,
    /// Class Pruner Multiplier 0\-255:       
    ///
    /// Example value: `"15"`
    ClassifyClassPrunerMultiplier,
    /// Class Pruner CutoffStrength:         
    ///
    /// Example value: `"7"`
    ClassifyCpCutoffStrength,
    /// Integer Matcher Multiplier  0\-255:   
    ///
    /// Example value: `"10"`
    ClassifyIntegerMatcherMultiplier,
    /// Set to 1 for general debug info, to 2 for more details, to 3 to see all the debug messages
    ///
    /// Example value: `"0"`
    DawgDebugLevel,
    /// Debug level for hyphenated words\.
    ///
    /// Example value: `"0"`
    HyphenDebugLevel,
    /// Size of dict word to be treated as non\-dict word
    ///
    /// Example value: `"2"`
    StopperSmallwordSize,
    /// Stopper debug level
    ///
    /// Example value: `"0"`
    StopperDebugLevel,
    /// Max words to keep in list
    ///
    /// Example value: `"10"`
    TesseditTruncateWordchoiceLog,
    /// Maximum number of different character choices to consider during permutation\. This limit is especially useful when user patterns are specified, since overly generic patterns can result in dawg search exploring an overly large number of options\.
    ///
    /// Example value: `"10000"`
    MaxPermuterAttempts,
    /// Fix blobs that aren't chopped
    ///
    /// Example value: `"1"`
    RepairUnchoppedBlobs,
    /// Chop debug
    ///
    /// Example value: `"0"`
    ChopDebug,
    /// Split Length
    ///
    /// Example value: `"10000"`
    ChopSplitLength,
    /// Same distance
    ///
    /// Example value: `"2"`
    ChopSameDistance,
    /// Min Number of Points on Outline
    ///
    /// Example value: `"6"`
    ChopMinOutlinePoints,
    /// Max number of seams in seam\_pile
    ///
    /// Example value: `"150"`
    ChopSeamPileSize,
    /// Min Inside Angle Bend
    ///
    /// Example value: `"-50"`
    ChopInsideAngle,
    /// Min Outline Area
    ///
    /// Example value: `"2000"`
    ChopMinOutlineArea,
    /// Width of \(smaller\) chopped blobs above which we don't care that a chop is not near the center\.
    ///
    /// Example value: `"90"`
    ChopCenteredMaxwidth,
    /// X / Y  length weight
    ///
    /// Example value: `"3"`
    ChopXyWeight,
    /// Debug level for wordrec
    ///
    /// Example value: `"0"`
    WordrecDebugLevel,
    /// Max number of broken pieces to associate
    ///
    /// Example value: `"4"`
    WordrecMaxJoinChunks,
    /// SegSearch debug level
    ///
    /// Example value: `"0"`
    SegsearchDebugLevel,
    /// Maximum number of pain points stored in the queue
    ///
    /// Example value: `"2000"`
    SegsearchMaxPainPoints,
    /// Maximum number of pain point classifications per chunk that did not result in finding a better word choice\.
    ///
    /// Example value: `"20"`
    SegsearchMaxFutileClassifications,
    /// Language model debug level
    ///
    /// Example value: `"0"`
    LanguageModelDebugLevel,
    /// Maximum order of the character ngram model
    ///
    /// Example value: `"8"`
    LanguageModelNgramOrder,
    /// Maximum number of prunable \(those for which PrunablePath\(\) is true\) entries in each viterbi list recorded in BLOB\_CHOICEs
    ///
    /// Example value: `"10"`
    LanguageModelViterbiListMaxNumPrunable,
    /// Maximum size of viterbi lists recorded in BLOB\_CHOICEs
    ///
    /// Example value: `"500"`
    LanguageModelViterbiListMaxSize,
    /// Minimum length of compound words
    ///
    /// Example value: `"3"`
    LanguageModelMinCompoundLength,
    /// Display Segmentations
    ///
    /// Example value: `"0"`
    WordrecDisplaySegmentations,
    /// Page seg mode: 0=osd only, 1=auto\+osd, 2=auto\_only, 3=auto, 4=column, 5=block\_vert, 6=block, 7=line, 8=word, 9=word\_circle, 10=char,11=sparse\_text, 12=sparse\_text\+osd, 13=raw\_line \(Values from PageSegMode enum in publictypes\.h\)
    ///
    /// Example value: `"6"`
    TesseditPagesegMode,
    /// Which OCR engine\(s\) to run \(Tesseract, LSTM, both\)\. Defaults to loading and running the most accurate available\.
    ///
    /// Example value: `"1"`
    TesseditOcrEngineMode,
    /// Whether to use the top\-line splitting process for Devanagari documents while performing page\-segmentation\.
    ///
    /// Example value: `"0"`
    PagesegDevanagariSplitStrategy,
    /// Whether to use the top\-line splitting process for Devanagari documents while performing ocr\.
    ///
    /// Example value: `"0"`
    OcrDevanagariSplitStrategy,
    /// Debug level for BiDi
    ///
    /// Example value: `"0"`
    BidiDebug,
    /// Debug level
    ///
    /// Example value: `"1"`
    ApplyboxDebug,
    /// Page number to apply boxes from
    ///
    /// Example value: `"0"`
    ApplyboxPage,
    /// Amount of debug output for bigram correction\.
    ///
    /// Example value: `"0"`
    TesseditBigramDebug,
    /// Debug reassignment of small outlines
    ///
    /// Example value: `"0"`
    DebugNoiseRemoval,
    /// Max diacritics to apply to a blob
    ///
    /// Example value: `"8"`
    NoiseMaxperblob,
    /// Max diacritics to apply to a word
    ///
    /// Example value: `"16"`
    NoiseMaxperword,
    /// Reestimate debug
    ///
    /// Example value: `"0"`
    DebugXHtLevel,
    /// alphas in a good word
    ///
    /// Example value: `"2"`
    QualityMinInitialAlphasReqd,
    /// Adaptation decision algorithm for tess
    ///
    /// Example value: `"39"`
    TesseditTessAdaptionMode,
    /// Print multilang debug info\.
    ///
    /// Example value: `"0"`
    MultilangDebugLevel,
    /// Print paragraph debug info\.
    ///
    /// Example value: `"0"`
    ParagraphDebugLevel,
    /// Only preserve wds longer than this
    ///
    /// Example value: `"2"`
    TesseditPreserveMinWdLen,
    /// For adj length in rating per ch
    ///
    /// Example value: `"10"`
    CrunchRatingMax,
    /// How many potential indicators needed
    ///
    /// Example value: `"1"`
    CrunchPotIndicators,
    /// Don't crunch words with long lower case strings
    ///
    /// Example value: `"4"`
    CrunchLeaveLcStrings,
    /// Don't crunch words with long lower case strings
    ///
    /// Example value: `"4"`
    CrunchLeaveUcStrings,
    /// Crunch words with long repetitions
    ///
    /// Example value: `"3"`
    CrunchLongRepetitions,
    /// As it says
    ///
    /// Example value: `"0"`
    CrunchDebug,
    /// How many non\-noise blbs either side?
    ///
    /// Example value: `"1"`
    FixspNonNoiseLimit,
    /// What constitues done for spacing
    ///
    /// Example value: `"1"`
    FixspDoneMode,
    /// Contextual fixspace debug
    ///
    /// Example value: `"0"`
    DebugFixSpaceLevel,
    /// Max allowed deviation of blob top outside of font data
    ///
    /// Example value: `"8"`
    XHtAcceptanceTolerance,
    /// Min change in xht before actually trying it
    ///
    /// Example value: `"8"`
    XHtMinChange,
    /// Debug level for sub & superscript fixer
    ///
    /// Example value: `"0"`
    SuperscriptDebug,
    /// Set JPEG quality level
    ///
    /// Example value: `"85"`
    JpgQuality,
    /// Specify DPI for input image
    ///
    /// Example value: `"0"`
    UserDefinedDpi,
    /// Specify minimum characters to try during OSD
    ///
    /// Example value: `"50"`
    MinCharactersToTry,
    /// Suspect marker level
    ///
    /// Example value: `"99"`
    SuspectLevel,
    /// Don't suspect dict wds longer than this
    ///
    /// Example value: `"2"`
    SuspectShortWords,
    /// Rejection algorithm
    ///
    /// Example value: `"0"`
    TesseditRejectMode,
    /// Rej blbs near image edge limit
    ///
    /// Example value: `"2"`
    TesseditImageBorder,
    /// Reject any x\-ht lt or eq than this
    ///
    /// Example value: `"8"`
    MinSaneXHtPixels,
    /// \-1 \-> All pages, else specific page to process
    ///
    /// Example value: `"-1"`
    TesseditPageNumber,
    /// Run in parallel where possible
    ///
    /// Example value: `"0"`
    TesseditParallelize,
    /// Allows to include alternative symbols choices in the hOCR output\. Valid input values are 0, 1, 2 and 3\. 0 is the default value\. With 1 the alternative symbol choices per timestep are included\. With 2 the alternative symbol choices are accumulated per character\. 
    ///
    /// Example value: `"0"`
    LstmChoiceMode,
    /// Debug data
    ///
    /// Example value: `"0"`
    TospDebugLevel,
    /// or should we use mean
    ///
    /// Example value: `"3"`
    TospEnoughSpaceSamplesForMedian,
    /// No\.samples reqd to reestimate for row
    ///
    /// Example value: `"10"`
    TospRedoKernLimit,
    /// No\.gaps reqd with 1 large gap to treat as a table
    ///
    /// Example value: `"40"`
    TospFewSamples,
    /// No\.gaps reqd with few cert spaces to use certs
    ///
    /// Example value: `"20"`
    TospShortRow,
    /// How to avoid being silly
    ///
    /// Example value: `"1"`
    TospSanityMethod,
    /// Pixel size of noise
    ///
    /// Example value: `"7"`
    TextordMaxNoiseSize,
    /// Baseline debug level
    ///
    /// Example value: `"0"`
    TextordBaselineDebug,
    /// Fraction of size for maxima
    ///
    /// Example value: `"10"`
    TextordNoiseSizefraction,
    /// Transitions for normal blob
    ///
    /// Example value: `"16"`
    TextordNoiseTranslimit,
    /// super norm blobs to save row
    ///
    /// Example value: `"1"`
    TextordNoiseSncount,
    /// Use ambigs for deciding whether to adapt to a character
    ///
    /// Example value: `"0"`
    UseAmbigsForAdaption,
    /// Use divisible blobs chopping
    ///
    /// Example value: `"1"`
    AllowBlobDivision,
    /// Prioritize blob division over chopping
    ///
    /// Example value: `"0"`
    PrioritizeDivision,
    /// Enable adaptive classifier
    ///
    /// Example value: `"1"`
    ClassifyEnableLearning,
    /// Character Normalized Matching
    ///
    /// Example value: `"0"`
    TessCnMatching,
    /// Baseline Normalized Matching
    ///
    /// Example value: `"0"`
    TessBnMatching,
    /// Enable adaptive classifier
    ///
    /// Example value: `"1"`
    ClassifyEnableAdaptiveMatcher,
    /// Use pre\-adapted classifier templates
    ///
    /// Example value: `"0"`
    ClassifyUsePreAdaptedTemplates,
    /// Save adapted templates to a file
    ///
    /// Example value: `"0"`
    ClassifySaveAdaptedTemplates,
    /// Enable match debugger
    ///
    /// Example value: `"0"`
    ClassifyEnableAdaptiveDebugger,
    /// Non\-linear stroke\-density normalization
    ///
    /// Example value: `"0"`
    ClassifyNonlinearNorm,
    /// Do not include character fragments in the results of the classifier
    ///
    /// Example value: `"1"`
    DisableCharacterFragments,
    /// Bring up graphical debugging windows for fragments training
    ///
    /// Example value: `"0"`
    ClassifyDebugCharacterFragments,
    /// Use two different windows for debugging the matching: One for the protos and one for the features\.
    ///
    /// Example value: `"0"`
    MatcherDebugSeparateWindows,
    /// Assume the input is numbers \[0\-9\]\.
    ///
    /// Example value: `"0"`
    ClassifyBlnNumericMode,
    /// Load system word dawg\.
    ///
    /// Example value: `"1"`
    LoadSystemDawg,
    /// Load frequent word dawg\.
    ///
    /// Example value: `"1"`
    LoadFreqDawg,
    /// Load unambiguous word dawg\.
    ///
    /// Example value: `"1"`
    LoadUnambigDawg,
    /// Load dawg with punctuation patterns\.
    ///
    /// Example value: `"1"`
    LoadPuncDawg,
    /// Load dawg with number patterns\.
    ///
    /// Example value: `"1"`
    LoadNumberDawg,
    /// Load dawg with special word bigrams\.
    ///
    /// Example value: `"1"`
    LoadBigramDawg,
    /// Use only the first UTF8 step of the given string when computing log probabilities\.
    ///
    /// Example value: `"0"`
    UseOnlyFirstUft8Step,
    /// Make AcceptableChoice\(\) always return false\. Useful when there is a need to explore all segmentations
    ///
    /// Example value: `"0"`
    StopperNoAcceptableChoices,
    /// Don't use any alphabetic\-specific tricks\. Set to true in the traineddata config file for scripts that are cursive or inherently fixed\-pitch
    ///
    /// Example value: `"0"`
    SegmentNonalphabeticScript,
    /// Save Document Words
    ///
    /// Example value: `"0"`
    SaveDocWords,
    /// Merge the fragments in the ratings matrix and delete them after merging
    ///
    /// Example value: `"1"`
    MergeFragmentsInMatrix,
    /// Associator Enable
    ///
    /// Example value: `"1"`
    WordrecEnableAssoc,
    /// force associator to run regardless of what enable\_assoc is\. This is used for CJK where component grouping is necessary\.
    ///
    /// Example value: `"0"`
    ForceWordAssoc,
    /// Chop enable
    ///
    /// Example value: `"1"`
    ChopEnable,
    /// Vertical creep
    ///
    /// Example value: `"0"`
    ChopVerticalCreep,
    /// Use new seam\_pile
    ///
    /// Example value: `"1"`
    ChopNewSeamPile,
    /// include fixed\-pitch heuristics in char segmentation
    ///
    /// Example value: `"0"`
    AssumeFixedPitchCharSegment,
    /// Only run OCR for words that had truth recorded in BlamerBundle
    ///
    /// Example value: `"0"`
    WordrecSkipNoTruthWords,
    /// Print blamer debug messages
    ///
    /// Example value: `"0"`
    WordrecDebugBlamer,
    /// Try to set the blame for errors
    ///
    /// Example value: `"0"`
    WordrecRunBlamer,
    /// Save alternative paths found during chopping and segmentation search
    ///
    /// Example value: `"1"`
    SaveAltChoices,
    /// Turn on/off the use of character ngram model
    ///
    /// Example value: `"0"`
    LanguageModelNgramOn,
    /// Use only the first UTF8 step of the given string when computing log probabilities\.
    ///
    /// Example value: `"0"`
    LanguageModelNgramUseOnlyFirstUft8Step,
    /// Words are delimited by space
    ///
    /// Example value: `"1"`
    LanguageModelNgramSpaceDelimitedLanguage,
    /// Use sigmoidal score for certainty
    ///
    /// Example value: `"0"`
    LanguageModelUseSigmoidalCertainty,
    /// Take segmentation and labeling from box file
    ///
    /// Example value: `"0"`
    TesseditResegmentFromBoxes,
    /// Conversion of word/line box file to char box file
    ///
    /// Example value: `"0"`
    TesseditResegmentFromLineBoxes,
    /// Generate training data from boxed chars
    ///
    /// Example value: `"0"`
    TesseditTrainFromBoxes,
    /// Generate more boxes from boxed chars
    ///
    /// Example value: `"0"`
    TesseditMakeBoxesFromBoxes,
    /// Break input into lines and remap boxes if present
    ///
    /// Example value: `"0"`
    TesseditTrainLineRecognizer,
    /// Dump intermediate images made during page segmentation
    ///
    /// Example value: `"0"`
    TesseditDumpPagesegImages,
    /// Try inverting the image in \`LSTMRecognizeWord\`
    ///
    /// Example value: `"1"`
    TesseditDoInvert,
    /// Perform training for ambiguities
    ///
    /// Example value: `"0"`
    TesseditAmbigsTraining,
    /// Generate and print debug information for adaption
    ///
    /// Example value: `"0"`
    TesseditAdaptionDebug,
    /// Learn both character fragments \(as is done in the special low exposure mode\) as well as unfragmented characters\.
    ///
    /// Example value: `"0"`
    ApplyboxLearnCharsAndCharFragsMode,
    /// Each bounding box is assumed to contain ngrams\. Only learn the ngrams whose outlines overlap horizontally\.
    ///
    /// Example value: `"0"`
    ApplyboxLearnNgramsMode,
    /// Draw output words
    ///
    /// Example value: `"0"`
    TesseditDisplayOutwords,
    /// Dump char choices
    ///
    /// Example value: `"0"`
    TesseditDumpChoices,
    /// Print timing stats
    ///
    /// Example value: `"0"`
    TesseditTimingDebug,
    /// Try to improve fuzzy spaces
    ///
    /// Example value: `"1"`
    TesseditFixFuzzySpaces,
    /// Don't bother with word plausibility
    ///
    /// Example value: `"0"`
    TesseditUnrejAnyWd,
    /// Crunch double hyphens?
    ///
    /// Example value: `"1"`
    TesseditFixHyphens,
    /// Add words to the document dictionary
    ///
    /// Example value: `"1"`
    TesseditEnableDocDict,
    /// Output font info per char
    ///
    /// Example value: `"0"`
    TesseditDebugFonts,
    /// Block and Row stats
    ///
    /// Example value: `"0"`
    TesseditDebugBlockRejection,
    /// Enable correction based on the word bigram dictionary\.
    ///
    /// Example value: `"1"`
    TesseditEnableBigramCorrection,
    /// Enable single word correction based on the dictionary\.
    ///
    /// Example value: `"0"`
    TesseditEnableDictCorrection,
    /// Remove and conditionally reassign small outlines when they confuse layout analysis, determining diacritics vs noise
    ///
    /// Example value: `"1"`
    EnableNoiseRemoval,
    /// Do minimal rejection on pass 1 output
    ///
    /// Example value: `"0"`
    TesseditMinimalRejPass1,
    /// Test adaption criteria
    ///
    /// Example value: `"0"`
    TesseditTestAdaption,
    /// Test for point
    ///
    /// Example value: `"0"`
    TestPt,
    /// Run paragraph detection on the post\-text\-recognition \(more accurate\)
    ///
    /// Example value: `"1"`
    ParagraphTextBased,
    /// Use ratings matrix/beam search with lstm
    ///
    /// Example value: `"1"`
    LstmUseMatrix,
    /// Reduce rejection on good docs
    ///
    /// Example value: `"1"`
    TesseditGoodQualityUnrej,
    /// Reject spaces?
    ///
    /// Example value: `"1"`
    TesseditUseRejectSpaces,
    /// Only rej partially rejected words in block rejection
    ///
    /// Example value: `"1"`
    TesseditPreserveBlkRejPerfectWds,
    /// Only rej partially rejected words in row rejection
    ///
    /// Example value: `"1"`
    TesseditPreserveRowRejPerfectWds,
    /// Use word segmentation quality metric
    ///
    /// Example value: `"0"`
    TesseditDontBlkrejGoodWds,
    /// Use word segmentation quality metric
    ///
    /// Example value: `"0"`
    TesseditDontRowrejGoodWds,
    /// Apply row rejection to good docs
    ///
    /// Example value: `"1"`
    TesseditRowRejGoodDocs,
    /// Reject all bad quality wds
    ///
    /// Example value: `"1"`
    TesseditRejectBadQualWds,
    /// Page stats
    ///
    /// Example value: `"0"`
    TesseditDebugDocRejection,
    /// Output data to debug file
    ///
    /// Example value: `"0"`
    TesseditDebugQualityMetrics,
    /// unrej potential with no checks
    ///
    /// Example value: `"0"`
    BlandUnrej,
    /// Mark v\.bad words for tilde crunch
    ///
    /// Example value: `"0"`
    UnlvTildeCrunching,
    /// Add font info to hocr output
    ///
    /// Example value: `"0"`
    HocrFontInfo,
    /// Add coordinates for each character to hocr output
    ///
    /// Example value: `"0"`
    HocrCharBoxes,
    /// Before word crunch?
    ///
    /// Example value: `"1"`
    CrunchEarlyMergeTessFails,
    /// Take out ~^ early?
    ///
    /// Example value: `"0"`
    CrunchEarlyConvertBadUnlvChs,
    /// As it says
    ///
    /// Example value: `"1"`
    CrunchTerribleGarbage,
    /// Don't touch sensible strings
    ///
    /// Example value: `"1"`
    CrunchLeaveOkStrings,
    /// Use acceptability in okstring
    ///
    /// Example value: `"1"`
    CrunchAcceptOk,
    /// Don't pot crunch sensible strings
    ///
    /// Example value: `"0"`
    CrunchLeaveAcceptStrings,
    /// Fiddle alpha figures
    ///
    /// Example value: `"0"`
    CrunchIncludeNumerals,
    /// Reward punctuation joins
    ///
    /// Example value: `"0"`
    TesseditPreferJoinedPunct,
    /// Write block separators in output
    ///
    /// Example value: `"0"`
    TesseditWriteBlockSeparators,
    /// Write repetition char code
    ///
    /// Example value: `"0"`
    TesseditWriteRepCodes,
    /// Write \.unlv output file
    ///
    /// Example value: `"0"`
    TesseditWriteUnlv,
    /// Write \.txt output file
    ///
    /// Example value: `"0"`
    TesseditCreateTxt,
    /// Write \.html hOCR output file
    ///
    /// Example value: `"0"`
    TesseditCreateHocr,
    /// Write \.xml ALTO file
    ///
    /// Example value: `"0"`
    TesseditCreateAlto,
    /// Write \.box file for LSTM training
    ///
    /// Example value: `"0"`
    TesseditCreateLstmbox,
    /// Write \.tsv output file
    ///
    /// Example value: `"0"`
    TesseditCreateTsv,
    /// Write WordStr format \.box output file
    ///
    /// Example value: `"0"`
    TesseditCreateWordstrbox,
    /// Write \.pdf output file
    ///
    /// Example value: `"0"`
    TesseditCreatePdf,
    /// Create PDF with only one invisible text layer
    ///
    /// Example value: `"0"`
    TextonlyPdf,
    /// UNLV keep 1Il chars rejected
    ///
    /// Example value: `"0"`
    SuspectConstrain1Il,
    /// Only reject tess failures
    ///
    /// Example value: `"0"`
    TesseditMinimalRejection,
    /// Don't reject ANYTHING
    ///
    /// Example value: `"0"`
    TesseditZeroRejection,
    /// Make output have exactly one word per WERD
    ///
    /// Example value: `"0"`
    TesseditWordForWord,
    /// Don't reject ANYTHING AT ALL
    ///
    /// Example value: `"0"`
    TesseditZeroKelvinRejection,
    /// Adaption debug
    ///
    /// Example value: `"0"`
    TesseditRejectionDebug,
    /// Contextual 0O O0 flips
    ///
    /// Example value: `"1"`
    TesseditFlip0O,
    /// Use DOC dawg in 11l conf\. detector
    ///
    /// Example value: `"0"`
    RejTrustDocDawg,
    /// Use dictword test
    ///
    /// Example value: `"0"`
    Rej1IlUseDictWord,
    /// Don't double check
    ///
    /// Example value: `"1"`
    Rej1IlTrustPermuterType,
    /// Individual rejection control
    ///
    /// Example value: `"1"`
    RejUseTessAccepted,
    /// Individual rejection control
    ///
    /// Example value: `"1"`
    RejUseTessBlanks,
    /// Individual rejection control
    ///
    /// Example value: `"1"`
    RejUseGoodPerm,
    /// Extend permuter check
    ///
    /// Example value: `"0"`
    RejUseSensibleWd,
    /// Extend permuter check
    ///
    /// Example value: `"0"`
    RejAlphasInNumberPerm,
    /// Output text with boxes
    ///
    /// Example value: `"0"`
    TesseditCreateBoxfile,
    /// Capture the image from the IPE
    ///
    /// Example value: `"0"`
    TesseditWriteImages,
    /// Run interactively?
    ///
    /// Example value: `"0"`
    InteractiveDisplayMode,
    /// According to dict\_word
    ///
    /// Example value: `"1"`
    TesseditOverridePermuter,
    /// In multilingual mode use params model of the primary language
    ///
    /// Example value: `"0"`
    TesseditUsePrimaryParamsModel,
    /// Debug line finding
    ///
    /// Example value: `"0"`
    TextordTabfindShowVlines,
    /// Use CJK fixed pitch model
    ///
    /// Example value: `"0"`
    TextordUseCjkFpModel,
    /// Allow feature extractors to see the original outline
    ///
    /// Example value: `"0"`
    PolyAllowDetailedFx,
    /// Only initialize with the config file\. Useful if the instance is not going to be used for OCR but say only for layout analysis\.
    ///
    /// Example value: `"0"`
    TesseditInitConfigOnly,
    /// Turn on equation detector
    ///
    /// Example value: `"0"`
    TextordEquationDetect,
    /// Enable vertical detection
    ///
    /// Example value: `"1"`
    TextordTabfindVerticalText,
    /// Force using vertical text page mode
    ///
    /// Example value: `"0"`
    TextordTabfindForceVerticalText,
    /// Preserve multiple interword spaces
    ///
    /// Example value: `"0"`
    PreserveInterwordSpaces,
    /// Detect music staff and remove intersecting components
    ///
    /// Example value: `"1"`
    PagesegApplyMusicMask,
    /// Script has no xheight, so use a single mode
    ///
    /// Example value: `"0"`
    TextordSingleHeightMode,
    /// Space stats use prechopping?
    ///
    /// Example value: `"0"`
    TospOldToMethod,
    /// Constrain relative values of inter and intra\-word gaps for old\_to\_method\.
    ///
    /// Example value: `"0"`
    TospOldToConstrainSpKn,
    /// Block stats to use fixed pitch rows?
    ///
    /// Example value: `"1"`
    TospOnlyUsePropRows,
    /// Force word breaks on punct to break long lines in non\-space delimited langs
    ///
    /// Example value: `"0"`
    TospForceWordbreakOnPunct,
    /// Space stats use prechopping?
    ///
    /// Example value: `"0"`
    TospUsePreChopping,
    /// Fix suspected bug in old code
    ///
    /// Example value: `"0"`
    TospOldToBugFix,
    /// Only stat OBVIOUS spaces
    ///
    /// Example value: `"1"`
    TospBlockUseCertSpaces,
    /// Only stat OBVIOUS spaces
    ///
    /// Example value: `"1"`
    TospRowUseCertSpaces,
    /// Only stat OBVIOUS spaces
    ///
    /// Example value: `"1"`
    TospNarrowBlobsNotCert,
    /// Only stat OBVIOUS spaces
    ///
    /// Example value: `"1"`
    TospRowUseCertSpaces1,
    /// Use row alone when inadequate cert spaces
    ///
    /// Example value: `"1"`
    TospRecoveryIsolatedRowStats,
    /// Better guess
    ///
    /// Example value: `"0"`
    TospOnlySmallGapsForKern,
    /// Pass ANY flip to context?
    ///
    /// Example value: `"0"`
    TospAllFlipsFuzzy,
    /// Don't restrict kn\->sp fuzzy limit to tables
    ///
    /// Example value: `"1"`
    TospFuzzyLimitAll,
    /// Use within xht gap for wd breaks
    ///
    /// Example value: `"1"`
    TospStatsUseXhtGaps,
    /// Use within xht gap for wd breaks
    ///
    /// Example value: `"1"`
    TospUseXhtGaps,
    /// Only use within xht gap for wd breaks
    ///
    /// Example value: `"0"`
    TospOnlyUseXhtGaps,
    /// Don't chng kn to space next to punct
    ///
    /// Example value: `"0"`
    TospRule9TestPunct,
    /// Default flip
    ///
    /// Example value: `"1"`
    TospFlipFuzzKnToSp,
    /// Default flip
    ///
    /// Example value: `"1"`
    TospFlipFuzzSpToKn,
    /// Enable improvement heuristic
    ///
    /// Example value: `"0"`
    TospImproveThresh,
    /// Don't remove noise blobs
    ///
    /// Example value: `"0"`
    TextordNoRejects,
    /// Display unsorted blobs
    ///
    /// Example value: `"0"`
    TextordShowBlobs,
    /// Display unsorted blobs
    ///
    /// Example value: `"0"`
    TextordShowBoxes,
    /// Reject noise\-like words
    ///
    /// Example value: `"1"`
    TextordNoiseRejwords,
    /// Reject noise\-like rows
    ///
    /// Example value: `"1"`
    TextordNoiseRejrows,
    /// Debug row garbage detector
    ///
    /// Example value: `"0"`
    TextordNoiseDebug,
    /// Class str to debug learning
    ClassifyLearnDebugStr,
    /// A filename of user\-provided words\.
    UserWordsFile,
    /// A suffix of user\-provided words located in tessdata\.
    UserWordsSuffix,
    /// A filename of user\-provided patterns\.
    UserPatternsFile,
    /// A suffix of user\-provided patterns located in tessdata\.
    UserPatternsSuffix,
    /// Output file for ambiguities found in the dictionary
    OutputAmbigWordsFile,
    /// Word for which stopper debug information should be printed to stdout
    WordToDebug,
    /// Blacklist of chars not to recognize
    TesseditCharBlacklist,
    /// Whitelist of chars to recognize
    TesseditCharWhitelist,
    /// List of chars to override tessedit\_char\_blacklist
    TesseditCharUnblacklist,
    /// Write all parameters to the given file\.
    TesseditWriteParamsToFile,
    /// Exposure value follows this pattern in the image filename\. The name of the image files are expected to be in the form \[lang\]\.\[fontname\]\.exp\[num\]\.tif
    ///
    /// Example value: `".exp"`
    ApplyboxExposurePattern,
    /// Leading punctuation
    ///
    /// Example value: `"(\'`\""`
    ChsLeadingPunct,
    /// 1st Trailing punctuation
    ///
    /// Example value: `").,;:?!"`
    ChsTrailingPunct1,
    /// 2nd Trailing punctuation
    ///
    /// Example value: `")\'`\""`
    ChsTrailingPunct2,
    /// Non standard number of outlines
    ///
    /// Example value: `"%| "`
    OutlinesOdd,
    /// Non standard number of outlines
    ///
    /// Example value: `"ij!?%\":;"`
    Outlines2,
    /// Punct\. chs expected WITHIN numbers
    ///
    /// Example value: `".,"`
    NumericPunctuation,
    /// Output char for unidentified blobs
    ///
    /// Example value: `"|"`
    UnrecognisedChar,
    /// Allow NN to unrej
    ///
    /// Example value: `"-?*="`
    OkRepeatedChNonAlphanumWds,
    /// Il1 conflict set
    ///
    /// Example value: `"Il1[]"`
    ConflictSetIl1,
    /// Filename extension
    ///
    /// Example value: `".tif"`
    FileType,
    /// List of languages to load with this one
    TesseditLoadSublangs,
    /// Page separator \(default is form feed control character\)
    ///
    /// Example value: `"\u{c}"`
    PageSeparator,
    /// Character Normalization Range \.\.\.
    ///
    /// Example value: `"0.2"`
    ClassifyCharNormRange,
    /// Veto ratio between classifier ratings
    ///
    /// Example value: `"1.5"`
    ClassifyMaxRatingRatio,
    /// Veto difference between classifier certainties
    ///
    /// Example value: `"5.5"`
    ClassifyMaxCertaintyMargin,
    /// Good Match \(0\-1\)
    ///
    /// Example value: `"0.125"`
    MatcherGoodThreshold,
    /// Great Match \(0\-1\)
    ///
    /// Example value: `"0"`
    MatcherReliableAdaptiveResult,
    /// Perfect Match \(0\-1\)
    ///
    /// Example value: `"0.02"`
    MatcherPerfectThreshold,
    /// Bad Match Pad \(0\-1\)
    ///
    /// Example value: `"0.15"`
    MatcherBadMatchPad,
    /// New template margin \(0\-1\)
    ///
    /// Example value: `"0.1"`
    MatcherRatingMargin,
    /// Avg\. noise blob length
    ///
    /// Example value: `"12"`
    MatcherAvgNoiseSize,
    /// Maximum angle delta for prototype clustering
    ///
    /// Example value: `"0.015"`
    MatcherClusteringMaxAngleDelta,
    /// Penalty to apply when a non\-alnum is vertically out of its expected textline position
    ///
    /// Example value: `"0"`
    ClassifyMisfitJunkPenalty,
    /// Rating scaling factor
    ///
    /// Example value: `"1.5"`
    RatingScale,
    /// Certainty scaling factor
    ///
    /// Example value: `"20"`
    CertaintyScale,
    /// Scale factor for features not used
    ///
    /// Example value: `"0.00390625"`
    TesseditClassMissScale,
    /// Prune poor adapted results this much worse than best result
    ///
    /// Example value: `"2.5"`
    ClassifyAdaptedPruningFactor,
    /// Threshold at which classify\_adapted\_pruning\_factor starts
    ///
    /// Example value: `"-1"`
    ClassifyAdaptedPruningThreshold,
    /// Exclude fragments that do not look like whole characters from training and adaption
    ///
    /// Example value: `"-3"`
    ClassifyCharacterFragmentsGarbageCertaintyThreshold,
    /// Max large speckle size
    ///
    /// Example value: `"0.3"`
    SpeckleLargeMaxSize,
    /// Penalty to add to worst rating for noise
    ///
    /// Example value: `"10"`
    SpeckleRatingPenalty,
    /// Score penalty \(0\.1 = 10%\) added if there are subscripts or superscripts in a word, but it is otherwise OK\.
    ///
    /// Example value: `"0.125"`
    XheightPenaltySubscripts,
    /// Score penalty \(0\.1 = 10%\) added if an xheight is inconsistent\.
    ///
    /// Example value: `"0.25"`
    XheightPenaltyInconsistent,
    /// Score multiplier for word matches which have good case and are frequent in the given language \(lower is better\)\.
    ///
    /// Example value: `"1"`
    SegmentPenaltyDictFrequentWord,
    /// Score multiplier for word matches that have good case \(lower is better\)\.
    ///
    /// Example value: `"1.1"`
    SegmentPenaltyDictCaseOk,
    /// Default score multiplier for word matches, which may have case issues \(lower is better\)\.
    ///
    /// Example value: `"1.3125"`
    SegmentPenaltyDictCaseBad,
    /// Score multiplier for glyph fragment segmentations which do not match a dictionary word \(lower is better\)\.
    ///
    /// Example value: `"1.25"`
    SegmentPenaltyDictNonword,
    /// Score multiplier for poorly cased strings that are not in the dictionary and generally look like garbage \(lower is better\)\.
    ///
    /// Example value: `"1.5"`
    SegmentPenaltyGarbage,
    /// Certainty threshold for non\-dict words
    ///
    /// Example value: `"-2.5"`
    StopperNondictCertaintyBase,
    /// Reject certainty offset
    ///
    /// Example value: `"1"`
    StopperPhase2CertaintyRejectionOffset,
    /// Certainty to add for each dict char above small word size\.
    ///
    /// Example value: `"-0.5"`
    StopperCertaintyPerChar,
    /// Max certaintly variation allowed in a word \(in sigma\)
    ///
    /// Example value: `"3"`
    StopperAllowableCharacterBadness,
    /// Worst certainty for using pending dictionary
    ///
    /// Example value: `"0"`
    DocDictPendingThreshold,
    /// Worst certainty for words that can be inserted into the document dictionary
    ///
    /// Example value: `"-2.25"`
    DocDictCertaintyThreshold,
    /// Good blob limit
    ///
    /// Example value: `"-2.25"`
    TesseditCertaintyThreshold,
    /// Split length adjustment
    ///
    /// Example value: `"0.5"`
    ChopSplitDistKnob,
    /// Split overlap adjustment
    ///
    /// Example value: `"0.9"`
    ChopOverlapKnob,
    /// Split center adjustment
    ///
    /// Example value: `"0.15"`
    ChopCenterKnob,
    /// Split sharpness adjustment
    ///
    /// Example value: `"0.06"`
    ChopSharpnessKnob,
    /// Width change adjustment
    ///
    /// Example value: `"5"`
    ChopWidthChangeKnob,
    /// OK split limit
    ///
    /// Example value: `"100"`
    ChopOkSplit,
    /// Good split limit
    ///
    /// Example value: `"50"`
    ChopGoodSplit,
    /// Maximum character width\-to\-height ratio
    ///
    /// Example value: `"2"`
    SegsearchMaxCharWhRatio,
    /// To avoid overly small denominators use this as the floor of the probability returned by the ngram model\.
    ///
    /// Example value: `"1e-06"`
    LanguageModelNgramSmallProb,
    /// Average classifier score of a non\-matching unichar\.
    ///
    /// Example value: `"-40"`
    LanguageModelNgramNonmatchScore,
    /// Strength of the character ngram model relative to the character classifier 
    ///
    /// Example value: `"0.03"`
    LanguageModelNgramScaleFactor,
    /// Factor to bring log\-probs into the same range as ratings when multiplied by outline length 
    ///
    /// Example value: `"16"`
    LanguageModelNgramRatingFactor,
    /// Penalty for words not in the frequent word dictionary
    ///
    /// Example value: `"0.1"`
    LanguageModelPenaltyNonFreqDictWord,
    /// Penalty for non\-dictionary words
    ///
    /// Example value: `"0.15"`
    LanguageModelPenaltyNonDictWord,
    /// Penalty for inconsistent punctuation
    ///
    /// Example value: `"0.2"`
    LanguageModelPenaltyPunc,
    /// Penalty for inconsistent case
    ///
    /// Example value: `"0.1"`
    LanguageModelPenaltyCase,
    /// Penalty for inconsistent script
    ///
    /// Example value: `"0.5"`
    LanguageModelPenaltyScript,
    /// Penalty for inconsistent character type
    ///
    /// Example value: `"0.3"`
    LanguageModelPenaltyChartype,
    /// Penalty for inconsistent font
    ///
    /// Example value: `"0"`
    LanguageModelPenaltyFont,
    /// Penalty for inconsistent spacing
    ///
    /// Example value: `"0.05"`
    LanguageModelPenaltySpacing,
    /// Penalty increment
    ///
    /// Example value: `"0.01"`
    LanguageModelPenaltyIncrement,
    /// Hingepoint for base char certainty
    ///
    /// Example value: `"-8"`
    NoiseCertBasechar,
    /// Hingepoint for disjoint certainty
    ///
    /// Example value: `"-1"`
    NoiseCertDisjoint,
    /// Threshold for new punc char certainty
    ///
    /// Example value: `"-3"`
    NoiseCertPunc,
    /// Scaling on certainty diff from Hingepoint
    ///
    /// Example value: `"0.375"`
    NoiseCertFactor,
    /// good\_quality\_doc lte rejection limit
    ///
    /// Example value: `"0.08"`
    QualityRejPc,
    /// good\_quality\_doc gte good blobs limit
    ///
    /// Example value: `"0"`
    QualityBlobPc,
    /// good\_quality\_doc lte outline error limit
    ///
    /// Example value: `"1"`
    QualityOutlinePc,
    /// good\_quality\_doc gte good char limit
    ///
    /// Example value: `"0.95"`
    QualityCharPc,
    /// xcoord
    ///
    /// Example value: `"100000"`
    TestPtX,
    /// ycoord
    ///
    /// Example value: `"100000"`
    TestPtY,
    /// %rej allowed before rej whole doc
    ///
    /// Example value: `"65"`
    TesseditRejectDocPercent,
    /// %rej allowed before rej whole block
    ///
    /// Example value: `"45"`
    TesseditRejectBlockPercent,
    /// %rej allowed before rej whole row
    ///
    /// Example value: `"40"`
    TesseditRejectRowPercent,
    /// Number of row rejects in whole word rejects which prevents whole row rejection
    ///
    /// Example value: `"70"`
    TesseditWholeWdRejRowPercent,
    /// rej good doc wd if more than this fraction rejected
    ///
    /// Example value: `"1.1"`
    TesseditGoodDocStillRowrejWd,
    /// good\_quality\_doc gte good char limit
    ///
    /// Example value: `"1.1"`
    QualityRowrejPc,
    /// crunch rating lt this
    ///
    /// Example value: `"80"`
    CrunchTerribleRating,
    /// crunch garbage cert lt this
    ///
    /// Example value: `"-9"`
    CrunchPoorGarbageCert,
    /// crunch garbage rating lt this
    ///
    /// Example value: `"60"`
    CrunchPoorGarbageRate,
    /// POTENTIAL crunch rating lt this
    ///
    /// Example value: `"40"`
    CrunchPotPoorRate,
    /// POTENTIAL crunch cert lt this
    ///
    /// Example value: `"-8"`
    CrunchPotPoorCert,
    /// POTENTIAL crunch rating lt this
    ///
    /// Example value: `"60"`
    CrunchDelRating,
    /// POTENTIAL crunch cert lt this
    ///
    /// Example value: `"-10"`
    CrunchDelCert,
    /// Del if word ht lt xht x this
    ///
    /// Example value: `"0.7"`
    CrunchDelMinHt,
    /// Del if word ht gt xht x this
    ///
    /// Example value: `"3"`
    CrunchDelMaxHt,
    /// Del if word width lt xht x this
    ///
    /// Example value: `"3"`
    CrunchDelMinWidth,
    /// Del if word gt xht x this above bl
    ///
    /// Example value: `"1.5"`
    CrunchDelHighWord,
    /// Del if word gt xht x this below bl
    ///
    /// Example value: `"0.5"`
    CrunchDelLowWord,
    /// Small if lt xht x this
    ///
    /// Example value: `"0.6"`
    CrunchSmallOutlinesSize,
    /// Small if lt xht x this
    ///
    /// Example value: `"0.28"`
    FixspSmallOutlinesSize,
    /// How many times worse certainty does a superscript position glyph need to be for us to try classifying it as a char with a different baseline?
    ///
    /// Example value: `"2"`
    SuperscriptWorseCertainty,
    /// What reduction in badness do we think sufficient to choose a superscript over what we'd thought\.  For example, a value of 0\.6 means we want to reduce badness of certainty by at least 40%
    ///
    /// Example value: `"0.97"`
    SuperscriptBetteredCertainty,
    /// A superscript scaled down more than this is unbelievably small\.  For example, 0\.3 means we expect the font size to be no smaller than 30% of the text line font size\.
    ///
    /// Example value: `"0.4"`
    SuperscriptScaledownRatio,
    /// Maximum top of a character measured as a multiple of x\-height above the baseline for us to reconsider whether it's a subscript\.
    ///
    /// Example value: `"0.5"`
    SubscriptMaxYTop,
    /// Minimum bottom of a character measured as a multiple of x\-height above the baseline for us to reconsider whether it's a superscript\.
    ///
    /// Example value: `"0.3"`
    SuperscriptMinYBottom,
    /// Don't touch bad rating limit
    ///
    /// Example value: `"999.9"`
    SuspectRatingPerCh,
    /// Accept good rating limit
    ///
    /// Example value: `"-999.9"`
    SuspectAcceptRating,
    /// Aspect ratio dot/hyphen test
    ///
    /// Example value: `"1.5"`
    TesseditLowerFlipHyphen,
    /// Aspect ratio dot/hyphen test
    ///
    /// Example value: `"1.8"`
    TesseditUpperFlipHyphen,
    /// if >this fract
    ///
    /// Example value: `"0.85"`
    RejWholeOfMostlyRejectWordFract,
    /// Min acceptable orientation margin
    ///
    /// Example value: `"7"`
    MinOrientationMargin,
    /// Fraction of textlines deemed vertical to use vertical page mode
    ///
    /// Example value: `"0.5"`
    TextordTabfindVerticalTextRatio,
    /// Fraction of height used as a minimum gap for aligned blobs\.
    ///
    /// Example value: `"0.75"`
    TextordTabfindAlignedGapFraction,
    /// Factor for defining space threshold in terms of space and kern sizes
    ///
    /// Example value: `"2"`
    TospOldSpKnThFactor,
    /// how far between kern and space?
    ///
    /// Example value: `"0"`
    TospThresholdBias1,
    /// how far between kern and space?
    ///
    /// Example value: `"0"`
    TospThresholdBias2,
    /// Fract of xheight for narrow
    ///
    /// Example value: `"0.3"`
    TospNarrowFraction,
    /// narrow if w/h less than this
    ///
    /// Example value: `"0.48"`
    TospNarrowAspectRatio,
    /// Fract of xheight for wide
    ///
    /// Example value: `"0.52"`
    TospWideFraction,
    /// wide if w/h less than this
    ///
    /// Example value: `"0"`
    TospWideAspectRatio,
    /// Fract of xheight for fuzz sp
    ///
    /// Example value: `"0.6"`
    TospFuzzySpaceFactor,
    /// Fract of xheight for fuzz sp
    ///
    /// Example value: `"0.5"`
    TospFuzzySpaceFactor1,
    /// Fract of xheight for fuzz sp
    ///
    /// Example value: `"0.72"`
    TospFuzzySpaceFactor2,
    /// gap ratio to flip sp\->kern
    ///
    /// Example value: `"0.83"`
    TospGapFactor,
    /// gap ratio to flip kern\->sp
    ///
    /// Example value: `"2"`
    TospKernGapFactor1,
    /// gap ratio to flip kern\->sp
    ///
    /// Example value: `"1.3"`
    TospKernGapFactor2,
    /// gap ratio to flip kern\->sp
    ///
    /// Example value: `"2.5"`
    TospKernGapFactor3,
    /// xht multiplier
    ///
    /// Example value: `"-1"`
    TospIgnoreBigGaps,
    /// xht multiplier
    ///
    /// Example value: `"3.5"`
    TospIgnoreVeryBigGaps,
    /// rep gap multiplier for space
    ///
    /// Example value: `"1.6"`
    TospRepSpace,
    /// Fract of kerns reqd for isolated row stats
    ///
    /// Example value: `"0.65"`
    TospEnoughSmallGaps,
    /// Min difference of kn & sp in table
    ///
    /// Example value: `"2.25"`
    TospTableKnSpRatio,
    /// Expect spaces bigger than this
    ///
    /// Example value: `"0.33"`
    TospTableXhtSpRatio,
    /// Fuzzy if less than this
    ///
    /// Example value: `"3"`
    TospTableFuzzyKnSpRatio,
    /// New fuzzy kn alg
    ///
    /// Example value: `"0.5"`
    TospFuzzyKnFraction,
    /// New fuzzy sp alg
    ///
    /// Example value: `"0.5"`
    TospFuzzySpFraction,
    /// Don't trust spaces less than this time kn
    ///
    /// Example value: `"1.5"`
    TospMinSaneKnSp,
    /// Thresh guess \- mult kn by this
    ///
    /// Example value: `"2.2"`
    TospInitGuessKnMult,
    /// Thresh guess \- mult xht by this
    ///
    /// Example value: `"0.28"`
    TospInitGuessXhtMult,
    /// Multiplier on kn to limit thresh
    ///
    /// Example value: `"5"`
    TospMaxSaneKnThresh,
    /// Don't autoflip kn to sp when large separation
    ///
    /// Example value: `"0"`
    TospFlipCaution,
    /// Limit use of xht gap with large kns
    ///
    /// Example value: `"0.19"`
    TospLargeKerning,
    /// Limit use of xht gap with odd small kns
    ///
    /// Example value: `"-1"`
    TospDontFoolWithSmallKerns,
    /// Don't reduce box if the top left is non blank
    ///
    /// Example value: `"0"`
    TospNearLhEdge,
    /// Don't let sp minus kn get too small
    ///
    /// Example value: `"0.2"`
    TospSillyKnSpGap,
    /// How wide fuzzies need context
    ///
    /// Example value: `"0.75"`
    TospPassWideFuzzSpToContext,
    /// Fraction of bounding box for noise
    ///
    /// Example value: `"0.7"`
    TextordNoiseAreaRatio,
    /// Ile of sizes for xheight guess
    ///
    /// Example value: `"0.75"`
    TextordInitialxIle,
    /// Ile of sizes for xheight guess
    ///
    /// Example value: `"0.9"`
    TextordInitialascIle,
    /// Fraction of x for big t count
    ///
    /// Example value: `"0.5"`
    TextordNoiseSizelimit,
    /// Dot to norm ratio for deletion
    ///
    /// Example value: `"2"`
    TextordNoiseNormratio,
    /// xh fract height error for norm blobs
    ///
    /// Example value: `"0.2"`
    TextordNoiseSyfract,
    /// xh fract width error for norm blobs
    ///
    /// Example value: `"0.4"`
    TextordNoiseSxfract,
    /// Height fraction to discard outlines as speckle noise
    ///
    /// Example value: `"0.015625"`
    TextordNoiseHfract,
    /// Dot to norm ratio for deletion
    ///
    /// Example value: `"6"`
    TextordNoiseRowratio,
    /// Max baseline shift
    ///
    /// Example value: `"0"`
    TextordBlshiftMaxshift,
    /// Min size of baseline shift
    ///
    /// Example value: `"9.99"`
    TextordBlshiftXfraction,
}

impl Variable {
    /// Get the variable's name as used by Tesseract
    pub fn as_cstr(&self) -> &'static CStr {
        match self {
            Variable::ClassifyNumCpLevels => CStr::from_bytes_with_nul(b"classify_num_cp_levels\0").unwrap(),
            Variable::TextordDotmatrixGap => CStr::from_bytes_with_nul(b"textord_dotmatrix_gap\0").unwrap(),
            Variable::TextordDebugBlock => CStr::from_bytes_with_nul(b"textord_debug_block\0").unwrap(),
            Variable::TextordPitchRange => CStr::from_bytes_with_nul(b"textord_pitch_range\0").unwrap(),
            Variable::TextordWordsVetoPower => CStr::from_bytes_with_nul(b"textord_words_veto_power\0").unwrap(),
            Variable::TextordTabfindShowStrokewidths => CStr::from_bytes_with_nul(b"textord_tabfind_show_strokewidths\0").unwrap(),
            Variable::PitsyncLinearVersion => CStr::from_bytes_with_nul(b"pitsync_linear_version\0").unwrap(),
            Variable::PitsyncFakeDepth => CStr::from_bytes_with_nul(b"pitsync_fake_depth\0").unwrap(),
            Variable::OldblHoledLosscount => CStr::from_bytes_with_nul(b"oldbl_holed_losscount\0").unwrap(),
            Variable::TextordSkewsmoothOffset => CStr::from_bytes_with_nul(b"textord_skewsmooth_offset\0").unwrap(),
            Variable::TextordSkewsmoothOffset2 => CStr::from_bytes_with_nul(b"textord_skewsmooth_offset2\0").unwrap(),
            Variable::TextordTestX => CStr::from_bytes_with_nul(b"textord_test_x\0").unwrap(),
            Variable::TextordTestY => CStr::from_bytes_with_nul(b"textord_test_y\0").unwrap(),
            Variable::TextordMinBlobsInRow => CStr::from_bytes_with_nul(b"textord_min_blobs_in_row\0").unwrap(),
            Variable::TextordSplineMinblobs => CStr::from_bytes_with_nul(b"textord_spline_minblobs\0").unwrap(),
            Variable::TextordSplineMedianwin => CStr::from_bytes_with_nul(b"textord_spline_medianwin\0").unwrap(),
            Variable::TextordMaxBlobOverlaps => CStr::from_bytes_with_nul(b"textord_max_blob_overlaps\0").unwrap(),
            Variable::TextordMinXheight => CStr::from_bytes_with_nul(b"textord_min_xheight\0").unwrap(),
            Variable::TextordLmsLineTrials => CStr::from_bytes_with_nul(b"textord_lms_line_trials\0").unwrap(),
            Variable::TextordTabfindShowImages => CStr::from_bytes_with_nul(b"textord_tabfind_show_images\0").unwrap(),
            Variable::TextordFpChopError => CStr::from_bytes_with_nul(b"textord_fp_chop_error\0").unwrap(),
            Variable::EdgesMaxChildrenPerOutline => CStr::from_bytes_with_nul(b"edges_max_children_per_outline\0").unwrap(),
            Variable::EdgesMaxChildrenLayers => CStr::from_bytes_with_nul(b"edges_max_children_layers\0").unwrap(),
            Variable::EdgesChildrenPerGrandchild => CStr::from_bytes_with_nul(b"edges_children_per_grandchild\0").unwrap(),
            Variable::EdgesChildrenCountLimit => CStr::from_bytes_with_nul(b"edges_children_count_limit\0").unwrap(),
            Variable::EdgesMinNonhole => CStr::from_bytes_with_nul(b"edges_min_nonhole\0").unwrap(),
            Variable::EdgesPathareaRatio => CStr::from_bytes_with_nul(b"edges_patharea_ratio\0").unwrap(),
            Variable::DevanagariSplitDebuglevel => CStr::from_bytes_with_nul(b"devanagari_split_debuglevel\0").unwrap(),
            Variable::TextordTabfindShowPartitions => CStr::from_bytes_with_nul(b"textord_tabfind_show_partitions\0").unwrap(),
            Variable::TextordDebugTabfind => CStr::from_bytes_with_nul(b"textord_debug_tabfind\0").unwrap(),
            Variable::TextordDebugBugs => CStr::from_bytes_with_nul(b"textord_debug_bugs\0").unwrap(),
            Variable::TextordTestregionLeft => CStr::from_bytes_with_nul(b"textord_testregion_left\0").unwrap(),
            Variable::TextordTestregionTop => CStr::from_bytes_with_nul(b"textord_testregion_top\0").unwrap(),
            Variable::TextordTestregionRight => CStr::from_bytes_with_nul(b"textord_testregion_right\0").unwrap(),
            Variable::TextordTestregionBottom => CStr::from_bytes_with_nul(b"textord_testregion_bottom\0").unwrap(),
            Variable::EditorImageXpos => CStr::from_bytes_with_nul(b"editor_image_xpos\0").unwrap(),
            Variable::EditorImageYpos => CStr::from_bytes_with_nul(b"editor_image_ypos\0").unwrap(),
            Variable::EditorImageMenuheight => CStr::from_bytes_with_nul(b"editor_image_menuheight\0").unwrap(),
            Variable::EditorImageWordBbColor => CStr::from_bytes_with_nul(b"editor_image_word_bb_color\0").unwrap(),
            Variable::EditorImageBlobBbColor => CStr::from_bytes_with_nul(b"editor_image_blob_bb_color\0").unwrap(),
            Variable::EditorImageTextColor => CStr::from_bytes_with_nul(b"editor_image_text_color\0").unwrap(),
            Variable::EditorDbwinXpos => CStr::from_bytes_with_nul(b"editor_dbwin_xpos\0").unwrap(),
            Variable::EditorDbwinYpos => CStr::from_bytes_with_nul(b"editor_dbwin_ypos\0").unwrap(),
            Variable::EditorDbwinHeight => CStr::from_bytes_with_nul(b"editor_dbwin_height\0").unwrap(),
            Variable::EditorDbwinWidth => CStr::from_bytes_with_nul(b"editor_dbwin_width\0").unwrap(),
            Variable::EditorWordXpos => CStr::from_bytes_with_nul(b"editor_word_xpos\0").unwrap(),
            Variable::EditorWordYpos => CStr::from_bytes_with_nul(b"editor_word_ypos\0").unwrap(),
            Variable::EditorWordHeight => CStr::from_bytes_with_nul(b"editor_word_height\0").unwrap(),
            Variable::EditorWordWidth => CStr::from_bytes_with_nul(b"editor_word_width\0").unwrap(),
            Variable::WordrecDisplaySplits => CStr::from_bytes_with_nul(b"wordrec_display_splits\0").unwrap(),
            Variable::PolyDebug => CStr::from_bytes_with_nul(b"poly_debug\0").unwrap(),
            Variable::PolyWideObjectsBetter => CStr::from_bytes_with_nul(b"poly_wide_objects_better\0").unwrap(),
            Variable::WordrecDisplayAllBlobs => CStr::from_bytes_with_nul(b"wordrec_display_all_blobs\0").unwrap(),
            Variable::WordrecBlobPause => CStr::from_bytes_with_nul(b"wordrec_blob_pause\0").unwrap(),
            Variable::TextordFpChopping => CStr::from_bytes_with_nul(b"textord_fp_chopping\0").unwrap(),
            Variable::TextordForceMakePropWords => CStr::from_bytes_with_nul(b"textord_force_make_prop_words\0").unwrap(),
            Variable::TextordChopperTest => CStr::from_bytes_with_nul(b"textord_chopper_test\0").unwrap(),
            Variable::TextordRestoreUnderlines => CStr::from_bytes_with_nul(b"textord_restore_underlines\0").unwrap(),
            Variable::TextordShowInitialWords => CStr::from_bytes_with_nul(b"textord_show_initial_words\0").unwrap(),
            Variable::TextordShowNewWords => CStr::from_bytes_with_nul(b"textord_show_new_words\0").unwrap(),
            Variable::TextordShowFixedWords => CStr::from_bytes_with_nul(b"textord_show_fixed_words\0").unwrap(),
            Variable::TextordBlocksallFixed => CStr::from_bytes_with_nul(b"textord_blocksall_fixed\0").unwrap(),
            Variable::TextordBlocksallProp => CStr::from_bytes_with_nul(b"textord_blocksall_prop\0").unwrap(),
            Variable::TextordBlocksallTesting => CStr::from_bytes_with_nul(b"textord_blocksall_testing\0").unwrap(),
            Variable::TextordTestMode => CStr::from_bytes_with_nul(b"textord_test_mode\0").unwrap(),
            Variable::TextordPitchScalebigwords => CStr::from_bytes_with_nul(b"textord_pitch_scalebigwords\0").unwrap(),
            Variable::TextordAllProp => CStr::from_bytes_with_nul(b"textord_all_prop\0").unwrap(),
            Variable::TextordDebugPitchTest => CStr::from_bytes_with_nul(b"textord_debug_pitch_test\0").unwrap(),
            Variable::TextordDisablePitchTest => CStr::from_bytes_with_nul(b"textord_disable_pitch_test\0").unwrap(),
            Variable::TextordFastPitchTest => CStr::from_bytes_with_nul(b"textord_fast_pitch_test\0").unwrap(),
            Variable::TextordDebugPitchMetric => CStr::from_bytes_with_nul(b"textord_debug_pitch_metric\0").unwrap(),
            Variable::TextordShowRowCuts => CStr::from_bytes_with_nul(b"textord_show_row_cuts\0").unwrap(),
            Variable::TextordShowPageCuts => CStr::from_bytes_with_nul(b"textord_show_page_cuts\0").unwrap(),
            Variable::TextordPitchCheat => CStr::from_bytes_with_nul(b"textord_pitch_cheat\0").unwrap(),
            Variable::TextordBlockndocFixed => CStr::from_bytes_with_nul(b"textord_blockndoc_fixed\0").unwrap(),
            Variable::TextordShowTables => CStr::from_bytes_with_nul(b"textord_show_tables\0").unwrap(),
            Variable::TextordTablefindShowMark => CStr::from_bytes_with_nul(b"textord_tablefind_show_mark\0").unwrap(),
            Variable::TextordTablefindShowStats => CStr::from_bytes_with_nul(b"textord_tablefind_show_stats\0").unwrap(),
            Variable::TextordTablefindRecognizeTables => CStr::from_bytes_with_nul(b"textord_tablefind_recognize_tables\0").unwrap(),
            Variable::TextordTabfindShowInitialtabs => CStr::from_bytes_with_nul(b"textord_tabfind_show_initialtabs\0").unwrap(),
            Variable::TextordTabfindShowFinaltabs => CStr::from_bytes_with_nul(b"textord_tabfind_show_finaltabs\0").unwrap(),
            Variable::TextordTabfindOnlyStrokewidths => CStr::from_bytes_with_nul(b"textord_tabfind_only_strokewidths\0").unwrap(),
            Variable::TextordReallyOldXheight => CStr::from_bytes_with_nul(b"textord_really_old_xheight\0").unwrap(),
            Variable::TextordOldblDebug => CStr::from_bytes_with_nul(b"textord_oldbl_debug\0").unwrap(),
            Variable::TextordDebugBaselines => CStr::from_bytes_with_nul(b"textord_debug_baselines\0").unwrap(),
            Variable::TextordOldblParadef => CStr::from_bytes_with_nul(b"textord_oldbl_paradef\0").unwrap(),
            Variable::TextordOldblSplitSplines => CStr::from_bytes_with_nul(b"textord_oldbl_split_splines\0").unwrap(),
            Variable::TextordOldblMergeParts => CStr::from_bytes_with_nul(b"textord_oldbl_merge_parts\0").unwrap(),
            Variable::OldblCorrfix => CStr::from_bytes_with_nul(b"oldbl_corrfix\0").unwrap(),
            Variable::OldblXhfix => CStr::from_bytes_with_nul(b"oldbl_xhfix\0").unwrap(),
            Variable::TextordOcropusMode => CStr::from_bytes_with_nul(b"textord_ocropus_mode\0").unwrap(),
            Variable::TextordHeavyNr => CStr::from_bytes_with_nul(b"textord_heavy_nr\0").unwrap(),
            Variable::TextordShowInitialRows => CStr::from_bytes_with_nul(b"textord_show_initial_rows\0").unwrap(),
            Variable::TextordShowParallelRows => CStr::from_bytes_with_nul(b"textord_show_parallel_rows\0").unwrap(),
            Variable::TextordShowExpandedRows => CStr::from_bytes_with_nul(b"textord_show_expanded_rows\0").unwrap(),
            Variable::TextordShowFinalRows => CStr::from_bytes_with_nul(b"textord_show_final_rows\0").unwrap(),
            Variable::TextordShowFinalBlobs => CStr::from_bytes_with_nul(b"textord_show_final_blobs\0").unwrap(),
            Variable::TextordTestLandscape => CStr::from_bytes_with_nul(b"textord_test_landscape\0").unwrap(),
            Variable::TextordParallelBaselines => CStr::from_bytes_with_nul(b"textord_parallel_baselines\0").unwrap(),
            Variable::TextordStraightBaselines => CStr::from_bytes_with_nul(b"textord_straight_baselines\0").unwrap(),
            Variable::TextordOldBaselines => CStr::from_bytes_with_nul(b"textord_old_baselines\0").unwrap(),
            Variable::TextordOldXheight => CStr::from_bytes_with_nul(b"textord_old_xheight\0").unwrap(),
            Variable::TextordFixXheightBug => CStr::from_bytes_with_nul(b"textord_fix_xheight_bug\0").unwrap(),
            Variable::TextordFixMakerowBug => CStr::from_bytes_with_nul(b"textord_fix_makerow_bug\0").unwrap(),
            Variable::TextordDebugXheights => CStr::from_bytes_with_nul(b"textord_debug_xheights\0").unwrap(),
            Variable::TextordBiasedSkewcalc => CStr::from_bytes_with_nul(b"textord_biased_skewcalc\0").unwrap(),
            Variable::TextordInterpolatingSkew => CStr::from_bytes_with_nul(b"textord_interpolating_skew\0").unwrap(),
            Variable::TextordNewInitialXheight => CStr::from_bytes_with_nul(b"textord_new_initial_xheight\0").unwrap(),
            Variable::TextordDebugBlob => CStr::from_bytes_with_nul(b"textord_debug_blob\0").unwrap(),
            Variable::GapmapDebug => CStr::from_bytes_with_nul(b"gapmap_debug\0").unwrap(),
            Variable::GapmapUseEnds => CStr::from_bytes_with_nul(b"gapmap_use_ends\0").unwrap(),
            Variable::GapmapNoIsolatedQuanta => CStr::from_bytes_with_nul(b"gapmap_no_isolated_quanta\0").unwrap(),
            Variable::EdgesUseNewOutlineComplexity => CStr::from_bytes_with_nul(b"edges_use_new_outline_complexity\0").unwrap(),
            Variable::EdgesDebug => CStr::from_bytes_with_nul(b"edges_debug\0").unwrap(),
            Variable::EdgesChildrenFix => CStr::from_bytes_with_nul(b"edges_children_fix\0").unwrap(),
            Variable::TextordShowFixedCuts => CStr::from_bytes_with_nul(b"textord_show_fixed_cuts\0").unwrap(),
            Variable::DevanagariSplitDebugimage => CStr::from_bytes_with_nul(b"devanagari_split_debugimage\0").unwrap(),
            Variable::TextordTabfindShowInitialPartitions => CStr::from_bytes_with_nul(b"textord_tabfind_show_initial_partitions\0").unwrap(),
            Variable::TextordTabfindShowRejectBlobs => CStr::from_bytes_with_nul(b"textord_tabfind_show_reject_blobs\0").unwrap(),
            Variable::TextordTabfindShowColumns => CStr::from_bytes_with_nul(b"textord_tabfind_show_columns\0").unwrap(),
            Variable::TextordTabfindShowBlocks => CStr::from_bytes_with_nul(b"textord_tabfind_show_blocks\0").unwrap(),
            Variable::TextordTabfindFindTables => CStr::from_bytes_with_nul(b"textord_tabfind_find_tables\0").unwrap(),
            Variable::TextordSpaceSizeIsVariable => CStr::from_bytes_with_nul(b"textord_space_size_is_variable\0").unwrap(),
            Variable::TextordDebugPrintable => CStr::from_bytes_with_nul(b"textord_debug_printable\0").unwrap(),
            Variable::EquationdetectSaveBiImage => CStr::from_bytes_with_nul(b"equationdetect_save_bi_image\0").unwrap(),
            Variable::EquationdetectSaveSptImage => CStr::from_bytes_with_nul(b"equationdetect_save_spt_image\0").unwrap(),
            Variable::EquationdetectSaveSeedImage => CStr::from_bytes_with_nul(b"equationdetect_save_seed_image\0").unwrap(),
            Variable::EquationdetectSaveMergedImage => CStr::from_bytes_with_nul(b"equationdetect_save_merged_image\0").unwrap(),
            Variable::StreamFilelist => CStr::from_bytes_with_nul(b"stream_filelist\0").unwrap(),
            Variable::DebugFile => CStr::from_bytes_with_nul(b"debug_file\0").unwrap(),
            Variable::Dotproduct => CStr::from_bytes_with_nul(b"dotproduct\0").unwrap(),
            Variable::ClassifyFontName => CStr::from_bytes_with_nul(b"classify_font_name\0").unwrap(),
            Variable::FxDebugfile => CStr::from_bytes_with_nul(b"fx_debugfile\0").unwrap(),
            Variable::EditorImageWinName => CStr::from_bytes_with_nul(b"editor_image_win_name\0").unwrap(),
            Variable::EditorDbwinName => CStr::from_bytes_with_nul(b"editor_dbwin_name\0").unwrap(),
            Variable::EditorWordName => CStr::from_bytes_with_nul(b"editor_word_name\0").unwrap(),
            Variable::DocumentTitle => CStr::from_bytes_with_nul(b"document_title\0").unwrap(),
            Variable::ClassifyPicoFeatureLength => CStr::from_bytes_with_nul(b"classify_pico_feature_length\0").unwrap(),
            Variable::ClassifyNormAdjMidpoint => CStr::from_bytes_with_nul(b"classify_norm_adj_midpoint\0").unwrap(),
            Variable::ClassifyNormAdjCurl => CStr::from_bytes_with_nul(b"classify_norm_adj_curl\0").unwrap(),
            Variable::ClassifyMinSlope => CStr::from_bytes_with_nul(b"classify_min_slope\0").unwrap(),
            Variable::ClassifyMaxSlope => CStr::from_bytes_with_nul(b"classify_max_slope\0").unwrap(),
            Variable::ClassifyCpAnglePadLoose => CStr::from_bytes_with_nul(b"classify_cp_angle_pad_loose\0").unwrap(),
            Variable::ClassifyCpAnglePadMedium => CStr::from_bytes_with_nul(b"classify_cp_angle_pad_medium\0").unwrap(),
            Variable::ClassifyCpAnglePadTight => CStr::from_bytes_with_nul(b"classify_cp_angle_pad_tight\0").unwrap(),
            Variable::ClassifyCpEndPadLoose => CStr::from_bytes_with_nul(b"classify_cp_end_pad_loose\0").unwrap(),
            Variable::ClassifyCpEndPadMedium => CStr::from_bytes_with_nul(b"classify_cp_end_pad_medium\0").unwrap(),
            Variable::ClassifyCpEndPadTight => CStr::from_bytes_with_nul(b"classify_cp_end_pad_tight\0").unwrap(),
            Variable::ClassifyCpSidePadLoose => CStr::from_bytes_with_nul(b"classify_cp_side_pad_loose\0").unwrap(),
            Variable::ClassifyCpSidePadMedium => CStr::from_bytes_with_nul(b"classify_cp_side_pad_medium\0").unwrap(),
            Variable::ClassifyCpSidePadTight => CStr::from_bytes_with_nul(b"classify_cp_side_pad_tight\0").unwrap(),
            Variable::ClassifyPpAnglePad => CStr::from_bytes_with_nul(b"classify_pp_angle_pad\0").unwrap(),
            Variable::ClassifyPpEndPad => CStr::from_bytes_with_nul(b"classify_pp_end_pad\0").unwrap(),
            Variable::ClassifyPpSidePad => CStr::from_bytes_with_nul(b"classify_pp_side_pad\0").unwrap(),
            Variable::TextordUnderlineOffset => CStr::from_bytes_with_nul(b"textord_underline_offset\0").unwrap(),
            Variable::TextordWordstatsSmoothFactor => CStr::from_bytes_with_nul(b"textord_wordstats_smooth_factor\0").unwrap(),
            Variable::TextordWidthSmoothFactor => CStr::from_bytes_with_nul(b"textord_width_smooth_factor\0").unwrap(),
            Variable::TextordWordsWidthIle => CStr::from_bytes_with_nul(b"textord_words_width_ile\0").unwrap(),
            Variable::TextordWordsMaxspace => CStr::from_bytes_with_nul(b"textord_words_maxspace\0").unwrap(),
            Variable::TextordWordsDefaultMaxspace => CStr::from_bytes_with_nul(b"textord_words_default_maxspace\0").unwrap(),
            Variable::TextordWordsDefaultMinspace => CStr::from_bytes_with_nul(b"textord_words_default_minspace\0").unwrap(),
            Variable::TextordWordsMinMinspace => CStr::from_bytes_with_nul(b"textord_words_min_minspace\0").unwrap(),
            Variable::TextordWordsDefaultNonspace => CStr::from_bytes_with_nul(b"textord_words_default_nonspace\0").unwrap(),
            Variable::TextordWordsInitialLower => CStr::from_bytes_with_nul(b"textord_words_initial_lower\0").unwrap(),
            Variable::TextordWordsInitialUpper => CStr::from_bytes_with_nul(b"textord_words_initial_upper\0").unwrap(),
            Variable::TextordWordsMinlarge => CStr::from_bytes_with_nul(b"textord_words_minlarge\0").unwrap(),
            Variable::TextordWordsPitchsdThreshold => CStr::from_bytes_with_nul(b"textord_words_pitchsd_threshold\0").unwrap(),
            Variable::TextordWordsDefFixed => CStr::from_bytes_with_nul(b"textord_words_def_fixed\0").unwrap(),
            Variable::TextordWordsDefProp => CStr::from_bytes_with_nul(b"textord_words_def_prop\0").unwrap(),
            Variable::TextordPitchRowsimilarity => CStr::from_bytes_with_nul(b"textord_pitch_rowsimilarity\0").unwrap(),
            Variable::WordsInitialLower => CStr::from_bytes_with_nul(b"words_initial_lower\0").unwrap(),
            Variable::WordsInitialUpper => CStr::from_bytes_with_nul(b"words_initial_upper\0").unwrap(),
            Variable::WordsDefaultPropNonspace => CStr::from_bytes_with_nul(b"words_default_prop_nonspace\0").unwrap(),
            Variable::WordsDefaultFixedSpace => CStr::from_bytes_with_nul(b"words_default_fixed_space\0").unwrap(),
            Variable::WordsDefaultFixedLimit => CStr::from_bytes_with_nul(b"words_default_fixed_limit\0").unwrap(),
            Variable::TextordWordsDefiniteSpread => CStr::from_bytes_with_nul(b"textord_words_definite_spread\0").unwrap(),
            Variable::TextordSpacesizeRatiofp => CStr::from_bytes_with_nul(b"textord_spacesize_ratiofp\0").unwrap(),
            Variable::TextordSpacesizeRatioprop => CStr::from_bytes_with_nul(b"textord_spacesize_ratioprop\0").unwrap(),
            Variable::TextordFpiqrRatio => CStr::from_bytes_with_nul(b"textord_fpiqr_ratio\0").unwrap(),
            Variable::TextordMaxPitchIqr => CStr::from_bytes_with_nul(b"textord_max_pitch_iqr\0").unwrap(),
            Variable::TextordFpMinWidth => CStr::from_bytes_with_nul(b"textord_fp_min_width\0").unwrap(),
            Variable::TextordProjectionScale => CStr::from_bytes_with_nul(b"textord_projection_scale\0").unwrap(),
            Variable::TextordBalanceFactor => CStr::from_bytes_with_nul(b"textord_balance_factor\0").unwrap(),
            Variable::TextordTabvectorVerticalGapFraction => CStr::from_bytes_with_nul(b"textord_tabvector_vertical_gap_fraction\0").unwrap(),
            Variable::TextordTabvectorVerticalBoxRatio => CStr::from_bytes_with_nul(b"textord_tabvector_vertical_box_ratio\0").unwrap(),
            Variable::PitsyncJoinedEdge => CStr::from_bytes_with_nul(b"pitsync_joined_edge\0").unwrap(),
            Variable::PitsyncOffsetFreecutFraction => CStr::from_bytes_with_nul(b"pitsync_offset_freecut_fraction\0").unwrap(),
            Variable::OldblXhfract => CStr::from_bytes_with_nul(b"oldbl_xhfract\0").unwrap(),
            Variable::OldblDotErrorSize => CStr::from_bytes_with_nul(b"oldbl_dot_error_size\0").unwrap(),
            Variable::TextordOldblJumplimit => CStr::from_bytes_with_nul(b"textord_oldbl_jumplimit\0").unwrap(),
            Variable::TextordSplineShiftFraction => CStr::from_bytes_with_nul(b"textord_spline_shift_fraction\0").unwrap(),
            Variable::TextordSplineOutlierFraction => CStr::from_bytes_with_nul(b"textord_spline_outlier_fraction\0").unwrap(),
            Variable::TextordSkewIle => CStr::from_bytes_with_nul(b"textord_skew_ile\0").unwrap(),
            Variable::TextordSkewLag => CStr::from_bytes_with_nul(b"textord_skew_lag\0").unwrap(),
            Variable::TextordLinespaceIqrlimit => CStr::from_bytes_with_nul(b"textord_linespace_iqrlimit\0").unwrap(),
            Variable::TextordWidthLimit => CStr::from_bytes_with_nul(b"textord_width_limit\0").unwrap(),
            Variable::TextordChopWidth => CStr::from_bytes_with_nul(b"textord_chop_width\0").unwrap(),
            Variable::TextordExpansionFactor => CStr::from_bytes_with_nul(b"textord_expansion_factor\0").unwrap(),
            Variable::TextordOverlapX => CStr::from_bytes_with_nul(b"textord_overlap_x\0").unwrap(),
            Variable::TextordMinxh => CStr::from_bytes_with_nul(b"textord_minxh\0").unwrap(),
            Variable::TextordMinLinesize => CStr::from_bytes_with_nul(b"textord_min_linesize\0").unwrap(),
            Variable::TextordExcessBlobsize => CStr::from_bytes_with_nul(b"textord_excess_blobsize\0").unwrap(),
            Variable::TextordOccupancyThreshold => CStr::from_bytes_with_nul(b"textord_occupancy_threshold\0").unwrap(),
            Variable::TextordUnderlineWidth => CStr::from_bytes_with_nul(b"textord_underline_width\0").unwrap(),
            Variable::TextordMinBlobHeightFraction => CStr::from_bytes_with_nul(b"textord_min_blob_height_fraction\0").unwrap(),
            Variable::TextordXheightModeFraction => CStr::from_bytes_with_nul(b"textord_xheight_mode_fraction\0").unwrap(),
            Variable::TextordAscheightModeFraction => CStr::from_bytes_with_nul(b"textord_ascheight_mode_fraction\0").unwrap(),
            Variable::TextordDescheightModeFraction => CStr::from_bytes_with_nul(b"textord_descheight_mode_fraction\0").unwrap(),
            Variable::TextordAscxRatioMin => CStr::from_bytes_with_nul(b"textord_ascx_ratio_min\0").unwrap(),
            Variable::TextordAscxRatioMax => CStr::from_bytes_with_nul(b"textord_ascx_ratio_max\0").unwrap(),
            Variable::TextordDescxRatioMin => CStr::from_bytes_with_nul(b"textord_descx_ratio_min\0").unwrap(),
            Variable::TextordDescxRatioMax => CStr::from_bytes_with_nul(b"textord_descx_ratio_max\0").unwrap(),
            Variable::TextordXheightErrorMargin => CStr::from_bytes_with_nul(b"textord_xheight_error_margin\0").unwrap(),
            Variable::GapmapBigGaps => CStr::from_bytes_with_nul(b"gapmap_big_gaps\0").unwrap(),
            Variable::TextordFpChopSnap => CStr::from_bytes_with_nul(b"textord_fp_chop_snap\0").unwrap(),
            Variable::EdgesChildarea => CStr::from_bytes_with_nul(b"edges_childarea\0").unwrap(),
            Variable::EdgesBoxarea => CStr::from_bytes_with_nul(b"edges_boxarea\0").unwrap(),
            Variable::TextordUnderlineThreshold => CStr::from_bytes_with_nul(b"textord_underline_threshold\0").unwrap(),
            Variable::AmbigsDebugLevel => CStr::from_bytes_with_nul(b"ambigs_debug_level\0").unwrap(),
            Variable::ClassifyDebugLevel => CStr::from_bytes_with_nul(b"classify_debug_level\0").unwrap(),
            Variable::ClassifyNormMethod => CStr::from_bytes_with_nul(b"classify_norm_method\0").unwrap(),
            Variable::MatcherDebugLevel => CStr::from_bytes_with_nul(b"matcher_debug_level\0").unwrap(),
            Variable::MatcherDebugFlags => CStr::from_bytes_with_nul(b"matcher_debug_flags\0").unwrap(),
            Variable::ClassifyLearningDebugLevel => CStr::from_bytes_with_nul(b"classify_learning_debug_level\0").unwrap(),
            Variable::MatcherPermanentClassesMin => CStr::from_bytes_with_nul(b"matcher_permanent_classes_min\0").unwrap(),
            Variable::MatcherMinExamplesForPrototyping => CStr::from_bytes_with_nul(b"matcher_min_examples_for_prototyping\0").unwrap(),
            Variable::MatcherSufficientExamplesForPrototyping => CStr::from_bytes_with_nul(b"matcher_sufficient_examples_for_prototyping\0").unwrap(),
            Variable::ClassifyAdaptProtoThreshold => CStr::from_bytes_with_nul(b"classify_adapt_proto_threshold\0").unwrap(),
            Variable::ClassifyAdaptFeatureThreshold => CStr::from_bytes_with_nul(b"classify_adapt_feature_threshold\0").unwrap(),
            Variable::ClassifyClassPrunerThreshold => CStr::from_bytes_with_nul(b"classify_class_pruner_threshold\0").unwrap(),
            Variable::ClassifyClassPrunerMultiplier => CStr::from_bytes_with_nul(b"classify_class_pruner_multiplier\0").unwrap(),
            Variable::ClassifyCpCutoffStrength => CStr::from_bytes_with_nul(b"classify_cp_cutoff_strength\0").unwrap(),
            Variable::ClassifyIntegerMatcherMultiplier => CStr::from_bytes_with_nul(b"classify_integer_matcher_multiplier\0").unwrap(),
            Variable::DawgDebugLevel => CStr::from_bytes_with_nul(b"dawg_debug_level\0").unwrap(),
            Variable::HyphenDebugLevel => CStr::from_bytes_with_nul(b"hyphen_debug_level\0").unwrap(),
            Variable::StopperSmallwordSize => CStr::from_bytes_with_nul(b"stopper_smallword_size\0").unwrap(),
            Variable::StopperDebugLevel => CStr::from_bytes_with_nul(b"stopper_debug_level\0").unwrap(),
            Variable::TesseditTruncateWordchoiceLog => CStr::from_bytes_with_nul(b"tessedit_truncate_wordchoice_log\0").unwrap(),
            Variable::MaxPermuterAttempts => CStr::from_bytes_with_nul(b"max_permuter_attempts\0").unwrap(),
            Variable::RepairUnchoppedBlobs => CStr::from_bytes_with_nul(b"repair_unchopped_blobs\0").unwrap(),
            Variable::ChopDebug => CStr::from_bytes_with_nul(b"chop_debug\0").unwrap(),
            Variable::ChopSplitLength => CStr::from_bytes_with_nul(b"chop_split_length\0").unwrap(),
            Variable::ChopSameDistance => CStr::from_bytes_with_nul(b"chop_same_distance\0").unwrap(),
            Variable::ChopMinOutlinePoints => CStr::from_bytes_with_nul(b"chop_min_outline_points\0").unwrap(),
            Variable::ChopSeamPileSize => CStr::from_bytes_with_nul(b"chop_seam_pile_size\0").unwrap(),
            Variable::ChopInsideAngle => CStr::from_bytes_with_nul(b"chop_inside_angle\0").unwrap(),
            Variable::ChopMinOutlineArea => CStr::from_bytes_with_nul(b"chop_min_outline_area\0").unwrap(),
            Variable::ChopCenteredMaxwidth => CStr::from_bytes_with_nul(b"chop_centered_maxwidth\0").unwrap(),
            Variable::ChopXyWeight => CStr::from_bytes_with_nul(b"chop_x_y_weight\0").unwrap(),
            Variable::WordrecDebugLevel => CStr::from_bytes_with_nul(b"wordrec_debug_level\0").unwrap(),
            Variable::WordrecMaxJoinChunks => CStr::from_bytes_with_nul(b"wordrec_max_join_chunks\0").unwrap(),
            Variable::SegsearchDebugLevel => CStr::from_bytes_with_nul(b"segsearch_debug_level\0").unwrap(),
            Variable::SegsearchMaxPainPoints => CStr::from_bytes_with_nul(b"segsearch_max_pain_points\0").unwrap(),
            Variable::SegsearchMaxFutileClassifications => CStr::from_bytes_with_nul(b"segsearch_max_futile_classifications\0").unwrap(),
            Variable::LanguageModelDebugLevel => CStr::from_bytes_with_nul(b"language_model_debug_level\0").unwrap(),
            Variable::LanguageModelNgramOrder => CStr::from_bytes_with_nul(b"language_model_ngram_order\0").unwrap(),
            Variable::LanguageModelViterbiListMaxNumPrunable => CStr::from_bytes_with_nul(b"language_model_viterbi_list_max_num_prunable\0").unwrap(),
            Variable::LanguageModelViterbiListMaxSize => CStr::from_bytes_with_nul(b"language_model_viterbi_list_max_size\0").unwrap(),
            Variable::LanguageModelMinCompoundLength => CStr::from_bytes_with_nul(b"language_model_min_compound_length\0").unwrap(),
            Variable::WordrecDisplaySegmentations => CStr::from_bytes_with_nul(b"wordrec_display_segmentations\0").unwrap(),
            Variable::TesseditPagesegMode => CStr::from_bytes_with_nul(b"tessedit_pageseg_mode\0").unwrap(),
            Variable::TesseditOcrEngineMode => CStr::from_bytes_with_nul(b"tessedit_ocr_engine_mode\0").unwrap(),
            Variable::PagesegDevanagariSplitStrategy => CStr::from_bytes_with_nul(b"pageseg_devanagari_split_strategy\0").unwrap(),
            Variable::OcrDevanagariSplitStrategy => CStr::from_bytes_with_nul(b"ocr_devanagari_split_strategy\0").unwrap(),
            Variable::BidiDebug => CStr::from_bytes_with_nul(b"bidi_debug\0").unwrap(),
            Variable::ApplyboxDebug => CStr::from_bytes_with_nul(b"applybox_debug\0").unwrap(),
            Variable::ApplyboxPage => CStr::from_bytes_with_nul(b"applybox_page\0").unwrap(),
            Variable::TesseditBigramDebug => CStr::from_bytes_with_nul(b"tessedit_bigram_debug\0").unwrap(),
            Variable::DebugNoiseRemoval => CStr::from_bytes_with_nul(b"debug_noise_removal\0").unwrap(),
            Variable::NoiseMaxperblob => CStr::from_bytes_with_nul(b"noise_maxperblob\0").unwrap(),
            Variable::NoiseMaxperword => CStr::from_bytes_with_nul(b"noise_maxperword\0").unwrap(),
            Variable::DebugXHtLevel => CStr::from_bytes_with_nul(b"debug_x_ht_level\0").unwrap(),
            Variable::QualityMinInitialAlphasReqd => CStr::from_bytes_with_nul(b"quality_min_initial_alphas_reqd\0").unwrap(),
            Variable::TesseditTessAdaptionMode => CStr::from_bytes_with_nul(b"tessedit_tess_adaption_mode\0").unwrap(),
            Variable::MultilangDebugLevel => CStr::from_bytes_with_nul(b"multilang_debug_level\0").unwrap(),
            Variable::ParagraphDebugLevel => CStr::from_bytes_with_nul(b"paragraph_debug_level\0").unwrap(),
            Variable::TesseditPreserveMinWdLen => CStr::from_bytes_with_nul(b"tessedit_preserve_min_wd_len\0").unwrap(),
            Variable::CrunchRatingMax => CStr::from_bytes_with_nul(b"crunch_rating_max\0").unwrap(),
            Variable::CrunchPotIndicators => CStr::from_bytes_with_nul(b"crunch_pot_indicators\0").unwrap(),
            Variable::CrunchLeaveLcStrings => CStr::from_bytes_with_nul(b"crunch_leave_lc_strings\0").unwrap(),
            Variable::CrunchLeaveUcStrings => CStr::from_bytes_with_nul(b"crunch_leave_uc_strings\0").unwrap(),
            Variable::CrunchLongRepetitions => CStr::from_bytes_with_nul(b"crunch_long_repetitions\0").unwrap(),
            Variable::CrunchDebug => CStr::from_bytes_with_nul(b"crunch_debug\0").unwrap(),
            Variable::FixspNonNoiseLimit => CStr::from_bytes_with_nul(b"fixsp_non_noise_limit\0").unwrap(),
            Variable::FixspDoneMode => CStr::from_bytes_with_nul(b"fixsp_done_mode\0").unwrap(),
            Variable::DebugFixSpaceLevel => CStr::from_bytes_with_nul(b"debug_fix_space_level\0").unwrap(),
            Variable::XHtAcceptanceTolerance => CStr::from_bytes_with_nul(b"x_ht_acceptance_tolerance\0").unwrap(),
            Variable::XHtMinChange => CStr::from_bytes_with_nul(b"x_ht_min_change\0").unwrap(),
            Variable::SuperscriptDebug => CStr::from_bytes_with_nul(b"superscript_debug\0").unwrap(),
            Variable::JpgQuality => CStr::from_bytes_with_nul(b"jpg_quality\0").unwrap(),
            Variable::UserDefinedDpi => CStr::from_bytes_with_nul(b"user_defined_dpi\0").unwrap(),
            Variable::MinCharactersToTry => CStr::from_bytes_with_nul(b"min_characters_to_try\0").unwrap(),
            Variable::SuspectLevel => CStr::from_bytes_with_nul(b"suspect_level\0").unwrap(),
            Variable::SuspectShortWords => CStr::from_bytes_with_nul(b"suspect_short_words\0").unwrap(),
            Variable::TesseditRejectMode => CStr::from_bytes_with_nul(b"tessedit_reject_mode\0").unwrap(),
            Variable::TesseditImageBorder => CStr::from_bytes_with_nul(b"tessedit_image_border\0").unwrap(),
            Variable::MinSaneXHtPixels => CStr::from_bytes_with_nul(b"min_sane_x_ht_pixels\0").unwrap(),
            Variable::TesseditPageNumber => CStr::from_bytes_with_nul(b"tessedit_page_number\0").unwrap(),
            Variable::TesseditParallelize => CStr::from_bytes_with_nul(b"tessedit_parallelize\0").unwrap(),
            Variable::LstmChoiceMode => CStr::from_bytes_with_nul(b"lstm_choice_mode\0").unwrap(),
            Variable::TospDebugLevel => CStr::from_bytes_with_nul(b"tosp_debug_level\0").unwrap(),
            Variable::TospEnoughSpaceSamplesForMedian => CStr::from_bytes_with_nul(b"tosp_enough_space_samples_for_median\0").unwrap(),
            Variable::TospRedoKernLimit => CStr::from_bytes_with_nul(b"tosp_redo_kern_limit\0").unwrap(),
            Variable::TospFewSamples => CStr::from_bytes_with_nul(b"tosp_few_samples\0").unwrap(),
            Variable::TospShortRow => CStr::from_bytes_with_nul(b"tosp_short_row\0").unwrap(),
            Variable::TospSanityMethod => CStr::from_bytes_with_nul(b"tosp_sanity_method\0").unwrap(),
            Variable::TextordMaxNoiseSize => CStr::from_bytes_with_nul(b"textord_max_noise_size\0").unwrap(),
            Variable::TextordBaselineDebug => CStr::from_bytes_with_nul(b"textord_baseline_debug\0").unwrap(),
            Variable::TextordNoiseSizefraction => CStr::from_bytes_with_nul(b"textord_noise_sizefraction\0").unwrap(),
            Variable::TextordNoiseTranslimit => CStr::from_bytes_with_nul(b"textord_noise_translimit\0").unwrap(),
            Variable::TextordNoiseSncount => CStr::from_bytes_with_nul(b"textord_noise_sncount\0").unwrap(),
            Variable::UseAmbigsForAdaption => CStr::from_bytes_with_nul(b"use_ambigs_for_adaption\0").unwrap(),
            Variable::AllowBlobDivision => CStr::from_bytes_with_nul(b"allow_blob_division\0").unwrap(),
            Variable::PrioritizeDivision => CStr::from_bytes_with_nul(b"prioritize_division\0").unwrap(),
            Variable::ClassifyEnableLearning => CStr::from_bytes_with_nul(b"classify_enable_learning\0").unwrap(),
            Variable::TessCnMatching => CStr::from_bytes_with_nul(b"tess_cn_matching\0").unwrap(),
            Variable::TessBnMatching => CStr::from_bytes_with_nul(b"tess_bn_matching\0").unwrap(),
            Variable::ClassifyEnableAdaptiveMatcher => CStr::from_bytes_with_nul(b"classify_enable_adaptive_matcher\0").unwrap(),
            Variable::ClassifyUsePreAdaptedTemplates => CStr::from_bytes_with_nul(b"classify_use_pre_adapted_templates\0").unwrap(),
            Variable::ClassifySaveAdaptedTemplates => CStr::from_bytes_with_nul(b"classify_save_adapted_templates\0").unwrap(),
            Variable::ClassifyEnableAdaptiveDebugger => CStr::from_bytes_with_nul(b"classify_enable_adaptive_debugger\0").unwrap(),
            Variable::ClassifyNonlinearNorm => CStr::from_bytes_with_nul(b"classify_nonlinear_norm\0").unwrap(),
            Variable::DisableCharacterFragments => CStr::from_bytes_with_nul(b"disable_character_fragments\0").unwrap(),
            Variable::ClassifyDebugCharacterFragments => CStr::from_bytes_with_nul(b"classify_debug_character_fragments\0").unwrap(),
            Variable::MatcherDebugSeparateWindows => CStr::from_bytes_with_nul(b"matcher_debug_separate_windows\0").unwrap(),
            Variable::ClassifyBlnNumericMode => CStr::from_bytes_with_nul(b"classify_bln_numeric_mode\0").unwrap(),
            Variable::LoadSystemDawg => CStr::from_bytes_with_nul(b"load_system_dawg\0").unwrap(),
            Variable::LoadFreqDawg => CStr::from_bytes_with_nul(b"load_freq_dawg\0").unwrap(),
            Variable::LoadUnambigDawg => CStr::from_bytes_with_nul(b"load_unambig_dawg\0").unwrap(),
            Variable::LoadPuncDawg => CStr::from_bytes_with_nul(b"load_punc_dawg\0").unwrap(),
            Variable::LoadNumberDawg => CStr::from_bytes_with_nul(b"load_number_dawg\0").unwrap(),
            Variable::LoadBigramDawg => CStr::from_bytes_with_nul(b"load_bigram_dawg\0").unwrap(),
            Variable::UseOnlyFirstUft8Step => CStr::from_bytes_with_nul(b"use_only_first_uft8_step\0").unwrap(),
            Variable::StopperNoAcceptableChoices => CStr::from_bytes_with_nul(b"stopper_no_acceptable_choices\0").unwrap(),
            Variable::SegmentNonalphabeticScript => CStr::from_bytes_with_nul(b"segment_nonalphabetic_script\0").unwrap(),
            Variable::SaveDocWords => CStr::from_bytes_with_nul(b"save_doc_words\0").unwrap(),
            Variable::MergeFragmentsInMatrix => CStr::from_bytes_with_nul(b"merge_fragments_in_matrix\0").unwrap(),
            Variable::WordrecEnableAssoc => CStr::from_bytes_with_nul(b"wordrec_enable_assoc\0").unwrap(),
            Variable::ForceWordAssoc => CStr::from_bytes_with_nul(b"force_word_assoc\0").unwrap(),
            Variable::ChopEnable => CStr::from_bytes_with_nul(b"chop_enable\0").unwrap(),
            Variable::ChopVerticalCreep => CStr::from_bytes_with_nul(b"chop_vertical_creep\0").unwrap(),
            Variable::ChopNewSeamPile => CStr::from_bytes_with_nul(b"chop_new_seam_pile\0").unwrap(),
            Variable::AssumeFixedPitchCharSegment => CStr::from_bytes_with_nul(b"assume_fixed_pitch_char_segment\0").unwrap(),
            Variable::WordrecSkipNoTruthWords => CStr::from_bytes_with_nul(b"wordrec_skip_no_truth_words\0").unwrap(),
            Variable::WordrecDebugBlamer => CStr::from_bytes_with_nul(b"wordrec_debug_blamer\0").unwrap(),
            Variable::WordrecRunBlamer => CStr::from_bytes_with_nul(b"wordrec_run_blamer\0").unwrap(),
            Variable::SaveAltChoices => CStr::from_bytes_with_nul(b"save_alt_choices\0").unwrap(),
            Variable::LanguageModelNgramOn => CStr::from_bytes_with_nul(b"language_model_ngram_on\0").unwrap(),
            Variable::LanguageModelNgramUseOnlyFirstUft8Step => CStr::from_bytes_with_nul(b"language_model_ngram_use_only_first_uft8_step\0").unwrap(),
            Variable::LanguageModelNgramSpaceDelimitedLanguage => CStr::from_bytes_with_nul(b"language_model_ngram_space_delimited_language\0").unwrap(),
            Variable::LanguageModelUseSigmoidalCertainty => CStr::from_bytes_with_nul(b"language_model_use_sigmoidal_certainty\0").unwrap(),
            Variable::TesseditResegmentFromBoxes => CStr::from_bytes_with_nul(b"tessedit_resegment_from_boxes\0").unwrap(),
            Variable::TesseditResegmentFromLineBoxes => CStr::from_bytes_with_nul(b"tessedit_resegment_from_line_boxes\0").unwrap(),
            Variable::TesseditTrainFromBoxes => CStr::from_bytes_with_nul(b"tessedit_train_from_boxes\0").unwrap(),
            Variable::TesseditMakeBoxesFromBoxes => CStr::from_bytes_with_nul(b"tessedit_make_boxes_from_boxes\0").unwrap(),
            Variable::TesseditTrainLineRecognizer => CStr::from_bytes_with_nul(b"tessedit_train_line_recognizer\0").unwrap(),
            Variable::TesseditDumpPagesegImages => CStr::from_bytes_with_nul(b"tessedit_dump_pageseg_images\0").unwrap(),
            Variable::TesseditDoInvert => CStr::from_bytes_with_nul(b"tessedit_do_invert\0").unwrap(),
            Variable::TesseditAmbigsTraining => CStr::from_bytes_with_nul(b"tessedit_ambigs_training\0").unwrap(),
            Variable::TesseditAdaptionDebug => CStr::from_bytes_with_nul(b"tessedit_adaption_debug\0").unwrap(),
            Variable::ApplyboxLearnCharsAndCharFragsMode => CStr::from_bytes_with_nul(b"applybox_learn_chars_and_char_frags_mode\0").unwrap(),
            Variable::ApplyboxLearnNgramsMode => CStr::from_bytes_with_nul(b"applybox_learn_ngrams_mode\0").unwrap(),
            Variable::TesseditDisplayOutwords => CStr::from_bytes_with_nul(b"tessedit_display_outwords\0").unwrap(),
            Variable::TesseditDumpChoices => CStr::from_bytes_with_nul(b"tessedit_dump_choices\0").unwrap(),
            Variable::TesseditTimingDebug => CStr::from_bytes_with_nul(b"tessedit_timing_debug\0").unwrap(),
            Variable::TesseditFixFuzzySpaces => CStr::from_bytes_with_nul(b"tessedit_fix_fuzzy_spaces\0").unwrap(),
            Variable::TesseditUnrejAnyWd => CStr::from_bytes_with_nul(b"tessedit_unrej_any_wd\0").unwrap(),
            Variable::TesseditFixHyphens => CStr::from_bytes_with_nul(b"tessedit_fix_hyphens\0").unwrap(),
            Variable::TesseditEnableDocDict => CStr::from_bytes_with_nul(b"tessedit_enable_doc_dict\0").unwrap(),
            Variable::TesseditDebugFonts => CStr::from_bytes_with_nul(b"tessedit_debug_fonts\0").unwrap(),
            Variable::TesseditDebugBlockRejection => CStr::from_bytes_with_nul(b"tessedit_debug_block_rejection\0").unwrap(),
            Variable::TesseditEnableBigramCorrection => CStr::from_bytes_with_nul(b"tessedit_enable_bigram_correction\0").unwrap(),
            Variable::TesseditEnableDictCorrection => CStr::from_bytes_with_nul(b"tessedit_enable_dict_correction\0").unwrap(),
            Variable::EnableNoiseRemoval => CStr::from_bytes_with_nul(b"enable_noise_removal\0").unwrap(),
            Variable::TesseditMinimalRejPass1 => CStr::from_bytes_with_nul(b"tessedit_minimal_rej_pass1\0").unwrap(),
            Variable::TesseditTestAdaption => CStr::from_bytes_with_nul(b"tessedit_test_adaption\0").unwrap(),
            Variable::TestPt => CStr::from_bytes_with_nul(b"test_pt\0").unwrap(),
            Variable::ParagraphTextBased => CStr::from_bytes_with_nul(b"paragraph_text_based\0").unwrap(),
            Variable::LstmUseMatrix => CStr::from_bytes_with_nul(b"lstm_use_matrix\0").unwrap(),
            Variable::TesseditGoodQualityUnrej => CStr::from_bytes_with_nul(b"tessedit_good_quality_unrej\0").unwrap(),
            Variable::TesseditUseRejectSpaces => CStr::from_bytes_with_nul(b"tessedit_use_reject_spaces\0").unwrap(),
            Variable::TesseditPreserveBlkRejPerfectWds => CStr::from_bytes_with_nul(b"tessedit_preserve_blk_rej_perfect_wds\0").unwrap(),
            Variable::TesseditPreserveRowRejPerfectWds => CStr::from_bytes_with_nul(b"tessedit_preserve_row_rej_perfect_wds\0").unwrap(),
            Variable::TesseditDontBlkrejGoodWds => CStr::from_bytes_with_nul(b"tessedit_dont_blkrej_good_wds\0").unwrap(),
            Variable::TesseditDontRowrejGoodWds => CStr::from_bytes_with_nul(b"tessedit_dont_rowrej_good_wds\0").unwrap(),
            Variable::TesseditRowRejGoodDocs => CStr::from_bytes_with_nul(b"tessedit_row_rej_good_docs\0").unwrap(),
            Variable::TesseditRejectBadQualWds => CStr::from_bytes_with_nul(b"tessedit_reject_bad_qual_wds\0").unwrap(),
            Variable::TesseditDebugDocRejection => CStr::from_bytes_with_nul(b"tessedit_debug_doc_rejection\0").unwrap(),
            Variable::TesseditDebugQualityMetrics => CStr::from_bytes_with_nul(b"tessedit_debug_quality_metrics\0").unwrap(),
            Variable::BlandUnrej => CStr::from_bytes_with_nul(b"bland_unrej\0").unwrap(),
            Variable::UnlvTildeCrunching => CStr::from_bytes_with_nul(b"unlv_tilde_crunching\0").unwrap(),
            Variable::HocrFontInfo => CStr::from_bytes_with_nul(b"hocr_font_info\0").unwrap(),
            Variable::HocrCharBoxes => CStr::from_bytes_with_nul(b"hocr_char_boxes\0").unwrap(),
            Variable::CrunchEarlyMergeTessFails => CStr::from_bytes_with_nul(b"crunch_early_merge_tess_fails\0").unwrap(),
            Variable::CrunchEarlyConvertBadUnlvChs => CStr::from_bytes_with_nul(b"crunch_early_convert_bad_unlv_chs\0").unwrap(),
            Variable::CrunchTerribleGarbage => CStr::from_bytes_with_nul(b"crunch_terrible_garbage\0").unwrap(),
            Variable::CrunchLeaveOkStrings => CStr::from_bytes_with_nul(b"crunch_leave_ok_strings\0").unwrap(),
            Variable::CrunchAcceptOk => CStr::from_bytes_with_nul(b"crunch_accept_ok\0").unwrap(),
            Variable::CrunchLeaveAcceptStrings => CStr::from_bytes_with_nul(b"crunch_leave_accept_strings\0").unwrap(),
            Variable::CrunchIncludeNumerals => CStr::from_bytes_with_nul(b"crunch_include_numerals\0").unwrap(),
            Variable::TesseditPreferJoinedPunct => CStr::from_bytes_with_nul(b"tessedit_prefer_joined_punct\0").unwrap(),
            Variable::TesseditWriteBlockSeparators => CStr::from_bytes_with_nul(b"tessedit_write_block_separators\0").unwrap(),
            Variable::TesseditWriteRepCodes => CStr::from_bytes_with_nul(b"tessedit_write_rep_codes\0").unwrap(),
            Variable::TesseditWriteUnlv => CStr::from_bytes_with_nul(b"tessedit_write_unlv\0").unwrap(),
            Variable::TesseditCreateTxt => CStr::from_bytes_with_nul(b"tessedit_create_txt\0").unwrap(),
            Variable::TesseditCreateHocr => CStr::from_bytes_with_nul(b"tessedit_create_hocr\0").unwrap(),
            Variable::TesseditCreateAlto => CStr::from_bytes_with_nul(b"tessedit_create_alto\0").unwrap(),
            Variable::TesseditCreateLstmbox => CStr::from_bytes_with_nul(b"tessedit_create_lstmbox\0").unwrap(),
            Variable::TesseditCreateTsv => CStr::from_bytes_with_nul(b"tessedit_create_tsv\0").unwrap(),
            Variable::TesseditCreateWordstrbox => CStr::from_bytes_with_nul(b"tessedit_create_wordstrbox\0").unwrap(),
            Variable::TesseditCreatePdf => CStr::from_bytes_with_nul(b"tessedit_create_pdf\0").unwrap(),
            Variable::TextonlyPdf => CStr::from_bytes_with_nul(b"textonly_pdf\0").unwrap(),
            Variable::SuspectConstrain1Il => CStr::from_bytes_with_nul(b"suspect_constrain_1Il\0").unwrap(),
            Variable::TesseditMinimalRejection => CStr::from_bytes_with_nul(b"tessedit_minimal_rejection\0").unwrap(),
            Variable::TesseditZeroRejection => CStr::from_bytes_with_nul(b"tessedit_zero_rejection\0").unwrap(),
            Variable::TesseditWordForWord => CStr::from_bytes_with_nul(b"tessedit_word_for_word\0").unwrap(),
            Variable::TesseditZeroKelvinRejection => CStr::from_bytes_with_nul(b"tessedit_zero_kelvin_rejection\0").unwrap(),
            Variable::TesseditRejectionDebug => CStr::from_bytes_with_nul(b"tessedit_rejection_debug\0").unwrap(),
            Variable::TesseditFlip0O => CStr::from_bytes_with_nul(b"tessedit_flip_0O\0").unwrap(),
            Variable::RejTrustDocDawg => CStr::from_bytes_with_nul(b"rej_trust_doc_dawg\0").unwrap(),
            Variable::Rej1IlUseDictWord => CStr::from_bytes_with_nul(b"rej_1Il_use_dict_word\0").unwrap(),
            Variable::Rej1IlTrustPermuterType => CStr::from_bytes_with_nul(b"rej_1Il_trust_permuter_type\0").unwrap(),
            Variable::RejUseTessAccepted => CStr::from_bytes_with_nul(b"rej_use_tess_accepted\0").unwrap(),
            Variable::RejUseTessBlanks => CStr::from_bytes_with_nul(b"rej_use_tess_blanks\0").unwrap(),
            Variable::RejUseGoodPerm => CStr::from_bytes_with_nul(b"rej_use_good_perm\0").unwrap(),
            Variable::RejUseSensibleWd => CStr::from_bytes_with_nul(b"rej_use_sensible_wd\0").unwrap(),
            Variable::RejAlphasInNumberPerm => CStr::from_bytes_with_nul(b"rej_alphas_in_number_perm\0").unwrap(),
            Variable::TesseditCreateBoxfile => CStr::from_bytes_with_nul(b"tessedit_create_boxfile\0").unwrap(),
            Variable::TesseditWriteImages => CStr::from_bytes_with_nul(b"tessedit_write_images\0").unwrap(),
            Variable::InteractiveDisplayMode => CStr::from_bytes_with_nul(b"interactive_display_mode\0").unwrap(),
            Variable::TesseditOverridePermuter => CStr::from_bytes_with_nul(b"tessedit_override_permuter\0").unwrap(),
            Variable::TesseditUsePrimaryParamsModel => CStr::from_bytes_with_nul(b"tessedit_use_primary_params_model\0").unwrap(),
            Variable::TextordTabfindShowVlines => CStr::from_bytes_with_nul(b"textord_tabfind_show_vlines\0").unwrap(),
            Variable::TextordUseCjkFpModel => CStr::from_bytes_with_nul(b"textord_use_cjk_fp_model\0").unwrap(),
            Variable::PolyAllowDetailedFx => CStr::from_bytes_with_nul(b"poly_allow_detailed_fx\0").unwrap(),
            Variable::TesseditInitConfigOnly => CStr::from_bytes_with_nul(b"tessedit_init_config_only\0").unwrap(),
            Variable::TextordEquationDetect => CStr::from_bytes_with_nul(b"textord_equation_detect\0").unwrap(),
            Variable::TextordTabfindVerticalText => CStr::from_bytes_with_nul(b"textord_tabfind_vertical_text\0").unwrap(),
            Variable::TextordTabfindForceVerticalText => CStr::from_bytes_with_nul(b"textord_tabfind_force_vertical_text\0").unwrap(),
            Variable::PreserveInterwordSpaces => CStr::from_bytes_with_nul(b"preserve_interword_spaces\0").unwrap(),
            Variable::PagesegApplyMusicMask => CStr::from_bytes_with_nul(b"pageseg_apply_music_mask\0").unwrap(),
            Variable::TextordSingleHeightMode => CStr::from_bytes_with_nul(b"textord_single_height_mode\0").unwrap(),
            Variable::TospOldToMethod => CStr::from_bytes_with_nul(b"tosp_old_to_method\0").unwrap(),
            Variable::TospOldToConstrainSpKn => CStr::from_bytes_with_nul(b"tosp_old_to_constrain_sp_kn\0").unwrap(),
            Variable::TospOnlyUsePropRows => CStr::from_bytes_with_nul(b"tosp_only_use_prop_rows\0").unwrap(),
            Variable::TospForceWordbreakOnPunct => CStr::from_bytes_with_nul(b"tosp_force_wordbreak_on_punct\0").unwrap(),
            Variable::TospUsePreChopping => CStr::from_bytes_with_nul(b"tosp_use_pre_chopping\0").unwrap(),
            Variable::TospOldToBugFix => CStr::from_bytes_with_nul(b"tosp_old_to_bug_fix\0").unwrap(),
            Variable::TospBlockUseCertSpaces => CStr::from_bytes_with_nul(b"tosp_block_use_cert_spaces\0").unwrap(),
            Variable::TospRowUseCertSpaces => CStr::from_bytes_with_nul(b"tosp_row_use_cert_spaces\0").unwrap(),
            Variable::TospNarrowBlobsNotCert => CStr::from_bytes_with_nul(b"tosp_narrow_blobs_not_cert\0").unwrap(),
            Variable::TospRowUseCertSpaces1 => CStr::from_bytes_with_nul(b"tosp_row_use_cert_spaces1\0").unwrap(),
            Variable::TospRecoveryIsolatedRowStats => CStr::from_bytes_with_nul(b"tosp_recovery_isolated_row_stats\0").unwrap(),
            Variable::TospOnlySmallGapsForKern => CStr::from_bytes_with_nul(b"tosp_only_small_gaps_for_kern\0").unwrap(),
            Variable::TospAllFlipsFuzzy => CStr::from_bytes_with_nul(b"tosp_all_flips_fuzzy\0").unwrap(),
            Variable::TospFuzzyLimitAll => CStr::from_bytes_with_nul(b"tosp_fuzzy_limit_all\0").unwrap(),
            Variable::TospStatsUseXhtGaps => CStr::from_bytes_with_nul(b"tosp_stats_use_xht_gaps\0").unwrap(),
            Variable::TospUseXhtGaps => CStr::from_bytes_with_nul(b"tosp_use_xht_gaps\0").unwrap(),
            Variable::TospOnlyUseXhtGaps => CStr::from_bytes_with_nul(b"tosp_only_use_xht_gaps\0").unwrap(),
            Variable::TospRule9TestPunct => CStr::from_bytes_with_nul(b"tosp_rule_9_test_punct\0").unwrap(),
            Variable::TospFlipFuzzKnToSp => CStr::from_bytes_with_nul(b"tosp_flip_fuzz_kn_to_sp\0").unwrap(),
            Variable::TospFlipFuzzSpToKn => CStr::from_bytes_with_nul(b"tosp_flip_fuzz_sp_to_kn\0").unwrap(),
            Variable::TospImproveThresh => CStr::from_bytes_with_nul(b"tosp_improve_thresh\0").unwrap(),
            Variable::TextordNoRejects => CStr::from_bytes_with_nul(b"textord_no_rejects\0").unwrap(),
            Variable::TextordShowBlobs => CStr::from_bytes_with_nul(b"textord_show_blobs\0").unwrap(),
            Variable::TextordShowBoxes => CStr::from_bytes_with_nul(b"textord_show_boxes\0").unwrap(),
            Variable::TextordNoiseRejwords => CStr::from_bytes_with_nul(b"textord_noise_rejwords\0").unwrap(),
            Variable::TextordNoiseRejrows => CStr::from_bytes_with_nul(b"textord_noise_rejrows\0").unwrap(),
            Variable::TextordNoiseDebug => CStr::from_bytes_with_nul(b"textord_noise_debug\0").unwrap(),
            Variable::ClassifyLearnDebugStr => CStr::from_bytes_with_nul(b"classify_learn_debug_str\0").unwrap(),
            Variable::UserWordsFile => CStr::from_bytes_with_nul(b"user_words_file\0").unwrap(),
            Variable::UserWordsSuffix => CStr::from_bytes_with_nul(b"user_words_suffix\0").unwrap(),
            Variable::UserPatternsFile => CStr::from_bytes_with_nul(b"user_patterns_file\0").unwrap(),
            Variable::UserPatternsSuffix => CStr::from_bytes_with_nul(b"user_patterns_suffix\0").unwrap(),
            Variable::OutputAmbigWordsFile => CStr::from_bytes_with_nul(b"output_ambig_words_file\0").unwrap(),
            Variable::WordToDebug => CStr::from_bytes_with_nul(b"word_to_debug\0").unwrap(),
            Variable::TesseditCharBlacklist => CStr::from_bytes_with_nul(b"tessedit_char_blacklist\0").unwrap(),
            Variable::TesseditCharWhitelist => CStr::from_bytes_with_nul(b"tessedit_char_whitelist\0").unwrap(),
            Variable::TesseditCharUnblacklist => CStr::from_bytes_with_nul(b"tessedit_char_unblacklist\0").unwrap(),
            Variable::TesseditWriteParamsToFile => CStr::from_bytes_with_nul(b"tessedit_write_params_to_file\0").unwrap(),
            Variable::ApplyboxExposurePattern => CStr::from_bytes_with_nul(b"applybox_exposure_pattern\0").unwrap(),
            Variable::ChsLeadingPunct => CStr::from_bytes_with_nul(b"chs_leading_punct\0").unwrap(),
            Variable::ChsTrailingPunct1 => CStr::from_bytes_with_nul(b"chs_trailing_punct1\0").unwrap(),
            Variable::ChsTrailingPunct2 => CStr::from_bytes_with_nul(b"chs_trailing_punct2\0").unwrap(),
            Variable::OutlinesOdd => CStr::from_bytes_with_nul(b"outlines_odd\0").unwrap(),
            Variable::Outlines2 => CStr::from_bytes_with_nul(b"outlines_2\0").unwrap(),
            Variable::NumericPunctuation => CStr::from_bytes_with_nul(b"numeric_punctuation\0").unwrap(),
            Variable::UnrecognisedChar => CStr::from_bytes_with_nul(b"unrecognised_char\0").unwrap(),
            Variable::OkRepeatedChNonAlphanumWds => CStr::from_bytes_with_nul(b"ok_repeated_ch_non_alphanum_wds\0").unwrap(),
            Variable::ConflictSetIl1 => CStr::from_bytes_with_nul(b"conflict_set_I_l_1\0").unwrap(),
            Variable::FileType => CStr::from_bytes_with_nul(b"file_type\0").unwrap(),
            Variable::TesseditLoadSublangs => CStr::from_bytes_with_nul(b"tessedit_load_sublangs\0").unwrap(),
            Variable::PageSeparator => CStr::from_bytes_with_nul(b"page_separator\0").unwrap(),
            Variable::ClassifyCharNormRange => CStr::from_bytes_with_nul(b"classify_char_norm_range\0").unwrap(),
            Variable::ClassifyMaxRatingRatio => CStr::from_bytes_with_nul(b"classify_max_rating_ratio\0").unwrap(),
            Variable::ClassifyMaxCertaintyMargin => CStr::from_bytes_with_nul(b"classify_max_certainty_margin\0").unwrap(),
            Variable::MatcherGoodThreshold => CStr::from_bytes_with_nul(b"matcher_good_threshold\0").unwrap(),
            Variable::MatcherReliableAdaptiveResult => CStr::from_bytes_with_nul(b"matcher_reliable_adaptive_result\0").unwrap(),
            Variable::MatcherPerfectThreshold => CStr::from_bytes_with_nul(b"matcher_perfect_threshold\0").unwrap(),
            Variable::MatcherBadMatchPad => CStr::from_bytes_with_nul(b"matcher_bad_match_pad\0").unwrap(),
            Variable::MatcherRatingMargin => CStr::from_bytes_with_nul(b"matcher_rating_margin\0").unwrap(),
            Variable::MatcherAvgNoiseSize => CStr::from_bytes_with_nul(b"matcher_avg_noise_size\0").unwrap(),
            Variable::MatcherClusteringMaxAngleDelta => CStr::from_bytes_with_nul(b"matcher_clustering_max_angle_delta\0").unwrap(),
            Variable::ClassifyMisfitJunkPenalty => CStr::from_bytes_with_nul(b"classify_misfit_junk_penalty\0").unwrap(),
            Variable::RatingScale => CStr::from_bytes_with_nul(b"rating_scale\0").unwrap(),
            Variable::CertaintyScale => CStr::from_bytes_with_nul(b"certainty_scale\0").unwrap(),
            Variable::TesseditClassMissScale => CStr::from_bytes_with_nul(b"tessedit_class_miss_scale\0").unwrap(),
            Variable::ClassifyAdaptedPruningFactor => CStr::from_bytes_with_nul(b"classify_adapted_pruning_factor\0").unwrap(),
            Variable::ClassifyAdaptedPruningThreshold => CStr::from_bytes_with_nul(b"classify_adapted_pruning_threshold\0").unwrap(),
            Variable::ClassifyCharacterFragmentsGarbageCertaintyThreshold => CStr::from_bytes_with_nul(b"classify_character_fragments_garbage_certainty_threshold\0").unwrap(),
            Variable::SpeckleLargeMaxSize => CStr::from_bytes_with_nul(b"speckle_large_max_size\0").unwrap(),
            Variable::SpeckleRatingPenalty => CStr::from_bytes_with_nul(b"speckle_rating_penalty\0").unwrap(),
            Variable::XheightPenaltySubscripts => CStr::from_bytes_with_nul(b"xheight_penalty_subscripts\0").unwrap(),
            Variable::XheightPenaltyInconsistent => CStr::from_bytes_with_nul(b"xheight_penalty_inconsistent\0").unwrap(),
            Variable::SegmentPenaltyDictFrequentWord => CStr::from_bytes_with_nul(b"segment_penalty_dict_frequent_word\0").unwrap(),
            Variable::SegmentPenaltyDictCaseOk => CStr::from_bytes_with_nul(b"segment_penalty_dict_case_ok\0").unwrap(),
            Variable::SegmentPenaltyDictCaseBad => CStr::from_bytes_with_nul(b"segment_penalty_dict_case_bad\0").unwrap(),
            Variable::SegmentPenaltyDictNonword => CStr::from_bytes_with_nul(b"segment_penalty_dict_nonword\0").unwrap(),
            Variable::SegmentPenaltyGarbage => CStr::from_bytes_with_nul(b"segment_penalty_garbage\0").unwrap(),
            Variable::StopperNondictCertaintyBase => CStr::from_bytes_with_nul(b"stopper_nondict_certainty_base\0").unwrap(),
            Variable::StopperPhase2CertaintyRejectionOffset => CStr::from_bytes_with_nul(b"stopper_phase2_certainty_rejection_offset\0").unwrap(),
            Variable::StopperCertaintyPerChar => CStr::from_bytes_with_nul(b"stopper_certainty_per_char\0").unwrap(),
            Variable::StopperAllowableCharacterBadness => CStr::from_bytes_with_nul(b"stopper_allowable_character_badness\0").unwrap(),
            Variable::DocDictPendingThreshold => CStr::from_bytes_with_nul(b"doc_dict_pending_threshold\0").unwrap(),
            Variable::DocDictCertaintyThreshold => CStr::from_bytes_with_nul(b"doc_dict_certainty_threshold\0").unwrap(),
            Variable::TesseditCertaintyThreshold => CStr::from_bytes_with_nul(b"tessedit_certainty_threshold\0").unwrap(),
            Variable::ChopSplitDistKnob => CStr::from_bytes_with_nul(b"chop_split_dist_knob\0").unwrap(),
            Variable::ChopOverlapKnob => CStr::from_bytes_with_nul(b"chop_overlap_knob\0").unwrap(),
            Variable::ChopCenterKnob => CStr::from_bytes_with_nul(b"chop_center_knob\0").unwrap(),
            Variable::ChopSharpnessKnob => CStr::from_bytes_with_nul(b"chop_sharpness_knob\0").unwrap(),
            Variable::ChopWidthChangeKnob => CStr::from_bytes_with_nul(b"chop_width_change_knob\0").unwrap(),
            Variable::ChopOkSplit => CStr::from_bytes_with_nul(b"chop_ok_split\0").unwrap(),
            Variable::ChopGoodSplit => CStr::from_bytes_with_nul(b"chop_good_split\0").unwrap(),
            Variable::SegsearchMaxCharWhRatio => CStr::from_bytes_with_nul(b"segsearch_max_char_wh_ratio\0").unwrap(),
            Variable::LanguageModelNgramSmallProb => CStr::from_bytes_with_nul(b"language_model_ngram_small_prob\0").unwrap(),
            Variable::LanguageModelNgramNonmatchScore => CStr::from_bytes_with_nul(b"language_model_ngram_nonmatch_score\0").unwrap(),
            Variable::LanguageModelNgramScaleFactor => CStr::from_bytes_with_nul(b"language_model_ngram_scale_factor\0").unwrap(),
            Variable::LanguageModelNgramRatingFactor => CStr::from_bytes_with_nul(b"language_model_ngram_rating_factor\0").unwrap(),
            Variable::LanguageModelPenaltyNonFreqDictWord => CStr::from_bytes_with_nul(b"language_model_penalty_non_freq_dict_word\0").unwrap(),
            Variable::LanguageModelPenaltyNonDictWord => CStr::from_bytes_with_nul(b"language_model_penalty_non_dict_word\0").unwrap(),
            Variable::LanguageModelPenaltyPunc => CStr::from_bytes_with_nul(b"language_model_penalty_punc\0").unwrap(),
            Variable::LanguageModelPenaltyCase => CStr::from_bytes_with_nul(b"language_model_penalty_case\0").unwrap(),
            Variable::LanguageModelPenaltyScript => CStr::from_bytes_with_nul(b"language_model_penalty_script\0").unwrap(),
            Variable::LanguageModelPenaltyChartype => CStr::from_bytes_with_nul(b"language_model_penalty_chartype\0").unwrap(),
            Variable::LanguageModelPenaltyFont => CStr::from_bytes_with_nul(b"language_model_penalty_font\0").unwrap(),
            Variable::LanguageModelPenaltySpacing => CStr::from_bytes_with_nul(b"language_model_penalty_spacing\0").unwrap(),
            Variable::LanguageModelPenaltyIncrement => CStr::from_bytes_with_nul(b"language_model_penalty_increment\0").unwrap(),
            Variable::NoiseCertBasechar => CStr::from_bytes_with_nul(b"noise_cert_basechar\0").unwrap(),
            Variable::NoiseCertDisjoint => CStr::from_bytes_with_nul(b"noise_cert_disjoint\0").unwrap(),
            Variable::NoiseCertPunc => CStr::from_bytes_with_nul(b"noise_cert_punc\0").unwrap(),
            Variable::NoiseCertFactor => CStr::from_bytes_with_nul(b"noise_cert_factor\0").unwrap(),
            Variable::QualityRejPc => CStr::from_bytes_with_nul(b"quality_rej_pc\0").unwrap(),
            Variable::QualityBlobPc => CStr::from_bytes_with_nul(b"quality_blob_pc\0").unwrap(),
            Variable::QualityOutlinePc => CStr::from_bytes_with_nul(b"quality_outline_pc\0").unwrap(),
            Variable::QualityCharPc => CStr::from_bytes_with_nul(b"quality_char_pc\0").unwrap(),
            Variable::TestPtX => CStr::from_bytes_with_nul(b"test_pt_x\0").unwrap(),
            Variable::TestPtY => CStr::from_bytes_with_nul(b"test_pt_y\0").unwrap(),
            Variable::TesseditRejectDocPercent => CStr::from_bytes_with_nul(b"tessedit_reject_doc_percent\0").unwrap(),
            Variable::TesseditRejectBlockPercent => CStr::from_bytes_with_nul(b"tessedit_reject_block_percent\0").unwrap(),
            Variable::TesseditRejectRowPercent => CStr::from_bytes_with_nul(b"tessedit_reject_row_percent\0").unwrap(),
            Variable::TesseditWholeWdRejRowPercent => CStr::from_bytes_with_nul(b"tessedit_whole_wd_rej_row_percent\0").unwrap(),
            Variable::TesseditGoodDocStillRowrejWd => CStr::from_bytes_with_nul(b"tessedit_good_doc_still_rowrej_wd\0").unwrap(),
            Variable::QualityRowrejPc => CStr::from_bytes_with_nul(b"quality_rowrej_pc\0").unwrap(),
            Variable::CrunchTerribleRating => CStr::from_bytes_with_nul(b"crunch_terrible_rating\0").unwrap(),
            Variable::CrunchPoorGarbageCert => CStr::from_bytes_with_nul(b"crunch_poor_garbage_cert\0").unwrap(),
            Variable::CrunchPoorGarbageRate => CStr::from_bytes_with_nul(b"crunch_poor_garbage_rate\0").unwrap(),
            Variable::CrunchPotPoorRate => CStr::from_bytes_with_nul(b"crunch_pot_poor_rate\0").unwrap(),
            Variable::CrunchPotPoorCert => CStr::from_bytes_with_nul(b"crunch_pot_poor_cert\0").unwrap(),
            Variable::CrunchDelRating => CStr::from_bytes_with_nul(b"crunch_del_rating\0").unwrap(),
            Variable::CrunchDelCert => CStr::from_bytes_with_nul(b"crunch_del_cert\0").unwrap(),
            Variable::CrunchDelMinHt => CStr::from_bytes_with_nul(b"crunch_del_min_ht\0").unwrap(),
            Variable::CrunchDelMaxHt => CStr::from_bytes_with_nul(b"crunch_del_max_ht\0").unwrap(),
            Variable::CrunchDelMinWidth => CStr::from_bytes_with_nul(b"crunch_del_min_width\0").unwrap(),
            Variable::CrunchDelHighWord => CStr::from_bytes_with_nul(b"crunch_del_high_word\0").unwrap(),
            Variable::CrunchDelLowWord => CStr::from_bytes_with_nul(b"crunch_del_low_word\0").unwrap(),
            Variable::CrunchSmallOutlinesSize => CStr::from_bytes_with_nul(b"crunch_small_outlines_size\0").unwrap(),
            Variable::FixspSmallOutlinesSize => CStr::from_bytes_with_nul(b"fixsp_small_outlines_size\0").unwrap(),
            Variable::SuperscriptWorseCertainty => CStr::from_bytes_with_nul(b"superscript_worse_certainty\0").unwrap(),
            Variable::SuperscriptBetteredCertainty => CStr::from_bytes_with_nul(b"superscript_bettered_certainty\0").unwrap(),
            Variable::SuperscriptScaledownRatio => CStr::from_bytes_with_nul(b"superscript_scaledown_ratio\0").unwrap(),
            Variable::SubscriptMaxYTop => CStr::from_bytes_with_nul(b"subscript_max_y_top\0").unwrap(),
            Variable::SuperscriptMinYBottom => CStr::from_bytes_with_nul(b"superscript_min_y_bottom\0").unwrap(),
            Variable::SuspectRatingPerCh => CStr::from_bytes_with_nul(b"suspect_rating_per_ch\0").unwrap(),
            Variable::SuspectAcceptRating => CStr::from_bytes_with_nul(b"suspect_accept_rating\0").unwrap(),
            Variable::TesseditLowerFlipHyphen => CStr::from_bytes_with_nul(b"tessedit_lower_flip_hyphen\0").unwrap(),
            Variable::TesseditUpperFlipHyphen => CStr::from_bytes_with_nul(b"tessedit_upper_flip_hyphen\0").unwrap(),
            Variable::RejWholeOfMostlyRejectWordFract => CStr::from_bytes_with_nul(b"rej_whole_of_mostly_reject_word_fract\0").unwrap(),
            Variable::MinOrientationMargin => CStr::from_bytes_with_nul(b"min_orientation_margin\0").unwrap(),
            Variable::TextordTabfindVerticalTextRatio => CStr::from_bytes_with_nul(b"textord_tabfind_vertical_text_ratio\0").unwrap(),
            Variable::TextordTabfindAlignedGapFraction => CStr::from_bytes_with_nul(b"textord_tabfind_aligned_gap_fraction\0").unwrap(),
            Variable::TospOldSpKnThFactor => CStr::from_bytes_with_nul(b"tosp_old_sp_kn_th_factor\0").unwrap(),
            Variable::TospThresholdBias1 => CStr::from_bytes_with_nul(b"tosp_threshold_bias1\0").unwrap(),
            Variable::TospThresholdBias2 => CStr::from_bytes_with_nul(b"tosp_threshold_bias2\0").unwrap(),
            Variable::TospNarrowFraction => CStr::from_bytes_with_nul(b"tosp_narrow_fraction\0").unwrap(),
            Variable::TospNarrowAspectRatio => CStr::from_bytes_with_nul(b"tosp_narrow_aspect_ratio\0").unwrap(),
            Variable::TospWideFraction => CStr::from_bytes_with_nul(b"tosp_wide_fraction\0").unwrap(),
            Variable::TospWideAspectRatio => CStr::from_bytes_with_nul(b"tosp_wide_aspect_ratio\0").unwrap(),
            Variable::TospFuzzySpaceFactor => CStr::from_bytes_with_nul(b"tosp_fuzzy_space_factor\0").unwrap(),
            Variable::TospFuzzySpaceFactor1 => CStr::from_bytes_with_nul(b"tosp_fuzzy_space_factor1\0").unwrap(),
            Variable::TospFuzzySpaceFactor2 => CStr::from_bytes_with_nul(b"tosp_fuzzy_space_factor2\0").unwrap(),
            Variable::TospGapFactor => CStr::from_bytes_with_nul(b"tosp_gap_factor\0").unwrap(),
            Variable::TospKernGapFactor1 => CStr::from_bytes_with_nul(b"tosp_kern_gap_factor1\0").unwrap(),
            Variable::TospKernGapFactor2 => CStr::from_bytes_with_nul(b"tosp_kern_gap_factor2\0").unwrap(),
            Variable::TospKernGapFactor3 => CStr::from_bytes_with_nul(b"tosp_kern_gap_factor3\0").unwrap(),
            Variable::TospIgnoreBigGaps => CStr::from_bytes_with_nul(b"tosp_ignore_big_gaps\0").unwrap(),
            Variable::TospIgnoreVeryBigGaps => CStr::from_bytes_with_nul(b"tosp_ignore_very_big_gaps\0").unwrap(),
            Variable::TospRepSpace => CStr::from_bytes_with_nul(b"tosp_rep_space\0").unwrap(),
            Variable::TospEnoughSmallGaps => CStr::from_bytes_with_nul(b"tosp_enough_small_gaps\0").unwrap(),
            Variable::TospTableKnSpRatio => CStr::from_bytes_with_nul(b"tosp_table_kn_sp_ratio\0").unwrap(),
            Variable::TospTableXhtSpRatio => CStr::from_bytes_with_nul(b"tosp_table_xht_sp_ratio\0").unwrap(),
            Variable::TospTableFuzzyKnSpRatio => CStr::from_bytes_with_nul(b"tosp_table_fuzzy_kn_sp_ratio\0").unwrap(),
            Variable::TospFuzzyKnFraction => CStr::from_bytes_with_nul(b"tosp_fuzzy_kn_fraction\0").unwrap(),
            Variable::TospFuzzySpFraction => CStr::from_bytes_with_nul(b"tosp_fuzzy_sp_fraction\0").unwrap(),
            Variable::TospMinSaneKnSp => CStr::from_bytes_with_nul(b"tosp_min_sane_kn_sp\0").unwrap(),
            Variable::TospInitGuessKnMult => CStr::from_bytes_with_nul(b"tosp_init_guess_kn_mult\0").unwrap(),
            Variable::TospInitGuessXhtMult => CStr::from_bytes_with_nul(b"tosp_init_guess_xht_mult\0").unwrap(),
            Variable::TospMaxSaneKnThresh => CStr::from_bytes_with_nul(b"tosp_max_sane_kn_thresh\0").unwrap(),
            Variable::TospFlipCaution => CStr::from_bytes_with_nul(b"tosp_flip_caution\0").unwrap(),
            Variable::TospLargeKerning => CStr::from_bytes_with_nul(b"tosp_large_kerning\0").unwrap(),
            Variable::TospDontFoolWithSmallKerns => CStr::from_bytes_with_nul(b"tosp_dont_fool_with_small_kerns\0").unwrap(),
            Variable::TospNearLhEdge => CStr::from_bytes_with_nul(b"tosp_near_lh_edge\0").unwrap(),
            Variable::TospSillyKnSpGap => CStr::from_bytes_with_nul(b"tosp_silly_kn_sp_gap\0").unwrap(),
            Variable::TospPassWideFuzzSpToContext => CStr::from_bytes_with_nul(b"tosp_pass_wide_fuzz_sp_to_context\0").unwrap(),
            Variable::TextordNoiseAreaRatio => CStr::from_bytes_with_nul(b"textord_noise_area_ratio\0").unwrap(),
            Variable::TextordInitialxIle => CStr::from_bytes_with_nul(b"textord_initialx_ile\0").unwrap(),
            Variable::TextordInitialascIle => CStr::from_bytes_with_nul(b"textord_initialasc_ile\0").unwrap(),
            Variable::TextordNoiseSizelimit => CStr::from_bytes_with_nul(b"textord_noise_sizelimit\0").unwrap(),
            Variable::TextordNoiseNormratio => CStr::from_bytes_with_nul(b"textord_noise_normratio\0").unwrap(),
            Variable::TextordNoiseSyfract => CStr::from_bytes_with_nul(b"textord_noise_syfract\0").unwrap(),
            Variable::TextordNoiseSxfract => CStr::from_bytes_with_nul(b"textord_noise_sxfract\0").unwrap(),
            Variable::TextordNoiseHfract => CStr::from_bytes_with_nul(b"textord_noise_hfract\0").unwrap(),
            Variable::TextordNoiseRowratio => CStr::from_bytes_with_nul(b"textord_noise_rowratio\0").unwrap(),
            Variable::TextordBlshiftMaxshift => CStr::from_bytes_with_nul(b"textord_blshift_maxshift\0").unwrap(),
            Variable::TextordBlshiftXfraction => CStr::from_bytes_with_nul(b"textord_blshift_xfraction\0").unwrap(),
        }
    }
}
