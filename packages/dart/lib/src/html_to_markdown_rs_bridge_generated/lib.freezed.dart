// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'lib.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;
/// @nodoc
mixin _$AnnotationKind {





@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is AnnotationKind);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'AnnotationKind()';
}


}

/// @nodoc
class $AnnotationKindCopyWith<$Res>  {
$AnnotationKindCopyWith(AnnotationKind _, $Res Function(AnnotationKind) __);
}


/// Adds pattern-matching-related methods to [AnnotationKind].
extension AnnotationKindPatterns on AnnotationKind {
/// A variant of `map` that fallback to returning `orElse`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( AnnotationKind_Bold value)?  bold,TResult Function( AnnotationKind_Italic value)?  italic,TResult Function( AnnotationKind_Underline value)?  underline,TResult Function( AnnotationKind_Strikethrough value)?  strikethrough,TResult Function( AnnotationKind_Code value)?  code,TResult Function( AnnotationKind_Subscript value)?  subscript,TResult Function( AnnotationKind_Superscript value)?  superscript,TResult Function( AnnotationKind_Highlight value)?  highlight,TResult Function( AnnotationKind_Link value)?  link,required TResult orElse(),}){
final _that = this;
switch (_that) {
case AnnotationKind_Bold() when bold != null:
return bold(_that);case AnnotationKind_Italic() when italic != null:
return italic(_that);case AnnotationKind_Underline() when underline != null:
return underline(_that);case AnnotationKind_Strikethrough() when strikethrough != null:
return strikethrough(_that);case AnnotationKind_Code() when code != null:
return code(_that);case AnnotationKind_Subscript() when subscript != null:
return subscript(_that);case AnnotationKind_Superscript() when superscript != null:
return superscript(_that);case AnnotationKind_Highlight() when highlight != null:
return highlight(_that);case AnnotationKind_Link() when link != null:
return link(_that);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// Callbacks receives the raw object, upcasted.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case final Subclass2 value:
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( AnnotationKind_Bold value)  bold,required TResult Function( AnnotationKind_Italic value)  italic,required TResult Function( AnnotationKind_Underline value)  underline,required TResult Function( AnnotationKind_Strikethrough value)  strikethrough,required TResult Function( AnnotationKind_Code value)  code,required TResult Function( AnnotationKind_Subscript value)  subscript,required TResult Function( AnnotationKind_Superscript value)  superscript,required TResult Function( AnnotationKind_Highlight value)  highlight,required TResult Function( AnnotationKind_Link value)  link,}){
final _that = this;
switch (_that) {
case AnnotationKind_Bold():
return bold(_that);case AnnotationKind_Italic():
return italic(_that);case AnnotationKind_Underline():
return underline(_that);case AnnotationKind_Strikethrough():
return strikethrough(_that);case AnnotationKind_Code():
return code(_that);case AnnotationKind_Subscript():
return subscript(_that);case AnnotationKind_Superscript():
return superscript(_that);case AnnotationKind_Highlight():
return highlight(_that);case AnnotationKind_Link():
return link(_that);}
}
/// A variant of `map` that fallback to returning `null`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( AnnotationKind_Bold value)?  bold,TResult? Function( AnnotationKind_Italic value)?  italic,TResult? Function( AnnotationKind_Underline value)?  underline,TResult? Function( AnnotationKind_Strikethrough value)?  strikethrough,TResult? Function( AnnotationKind_Code value)?  code,TResult? Function( AnnotationKind_Subscript value)?  subscript,TResult? Function( AnnotationKind_Superscript value)?  superscript,TResult? Function( AnnotationKind_Highlight value)?  highlight,TResult? Function( AnnotationKind_Link value)?  link,}){
final _that = this;
switch (_that) {
case AnnotationKind_Bold() when bold != null:
return bold(_that);case AnnotationKind_Italic() when italic != null:
return italic(_that);case AnnotationKind_Underline() when underline != null:
return underline(_that);case AnnotationKind_Strikethrough() when strikethrough != null:
return strikethrough(_that);case AnnotationKind_Code() when code != null:
return code(_that);case AnnotationKind_Subscript() when subscript != null:
return subscript(_that);case AnnotationKind_Superscript() when superscript != null:
return superscript(_that);case AnnotationKind_Highlight() when highlight != null:
return highlight(_that);case AnnotationKind_Link() when link != null:
return link(_that);case _:
  return null;

}
}
/// A variant of `when` that fallback to an `orElse` callback.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function()?  bold,TResult Function()?  italic,TResult Function()?  underline,TResult Function()?  strikethrough,TResult Function()?  code,TResult Function()?  subscript,TResult Function()?  superscript,TResult Function()?  highlight,TResult Function( String url,  String title)?  link,required TResult orElse(),}) {final _that = this;
switch (_that) {
case AnnotationKind_Bold() when bold != null:
return bold();case AnnotationKind_Italic() when italic != null:
return italic();case AnnotationKind_Underline() when underline != null:
return underline();case AnnotationKind_Strikethrough() when strikethrough != null:
return strikethrough();case AnnotationKind_Code() when code != null:
return code();case AnnotationKind_Subscript() when subscript != null:
return subscript();case AnnotationKind_Superscript() when superscript != null:
return superscript();case AnnotationKind_Highlight() when highlight != null:
return highlight();case AnnotationKind_Link() when link != null:
return link(_that.url,_that.title);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// As opposed to `map`, this offers destructuring.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case Subclass2(:final field2):
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function()  bold,required TResult Function()  italic,required TResult Function()  underline,required TResult Function()  strikethrough,required TResult Function()  code,required TResult Function()  subscript,required TResult Function()  superscript,required TResult Function()  highlight,required TResult Function( String url,  String title)  link,}) {final _that = this;
switch (_that) {
case AnnotationKind_Bold():
return bold();case AnnotationKind_Italic():
return italic();case AnnotationKind_Underline():
return underline();case AnnotationKind_Strikethrough():
return strikethrough();case AnnotationKind_Code():
return code();case AnnotationKind_Subscript():
return subscript();case AnnotationKind_Superscript():
return superscript();case AnnotationKind_Highlight():
return highlight();case AnnotationKind_Link():
return link(_that.url,_that.title);}
}
/// A variant of `when` that fallback to returning `null`
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function()?  bold,TResult? Function()?  italic,TResult? Function()?  underline,TResult? Function()?  strikethrough,TResult? Function()?  code,TResult? Function()?  subscript,TResult? Function()?  superscript,TResult? Function()?  highlight,TResult? Function( String url,  String title)?  link,}) {final _that = this;
switch (_that) {
case AnnotationKind_Bold() when bold != null:
return bold();case AnnotationKind_Italic() when italic != null:
return italic();case AnnotationKind_Underline() when underline != null:
return underline();case AnnotationKind_Strikethrough() when strikethrough != null:
return strikethrough();case AnnotationKind_Code() when code != null:
return code();case AnnotationKind_Subscript() when subscript != null:
return subscript();case AnnotationKind_Superscript() when superscript != null:
return superscript();case AnnotationKind_Highlight() when highlight != null:
return highlight();case AnnotationKind_Link() when link != null:
return link(_that.url,_that.title);case _:
  return null;

}
}

}

/// @nodoc


class AnnotationKind_Bold extends AnnotationKind {
  const AnnotationKind_Bold(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is AnnotationKind_Bold);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'AnnotationKind.bold()';
}


}




/// @nodoc


class AnnotationKind_Italic extends AnnotationKind {
  const AnnotationKind_Italic(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is AnnotationKind_Italic);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'AnnotationKind.italic()';
}


}




/// @nodoc


class AnnotationKind_Underline extends AnnotationKind {
  const AnnotationKind_Underline(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is AnnotationKind_Underline);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'AnnotationKind.underline()';
}


}




/// @nodoc


class AnnotationKind_Strikethrough extends AnnotationKind {
  const AnnotationKind_Strikethrough(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is AnnotationKind_Strikethrough);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'AnnotationKind.strikethrough()';
}


}




/// @nodoc


class AnnotationKind_Code extends AnnotationKind {
  const AnnotationKind_Code(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is AnnotationKind_Code);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'AnnotationKind.code()';
}


}




/// @nodoc


class AnnotationKind_Subscript extends AnnotationKind {
  const AnnotationKind_Subscript(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is AnnotationKind_Subscript);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'AnnotationKind.subscript()';
}


}




/// @nodoc


class AnnotationKind_Superscript extends AnnotationKind {
  const AnnotationKind_Superscript(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is AnnotationKind_Superscript);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'AnnotationKind.superscript()';
}


}




/// @nodoc


class AnnotationKind_Highlight extends AnnotationKind {
  const AnnotationKind_Highlight(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is AnnotationKind_Highlight);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'AnnotationKind.highlight()';
}


}




/// @nodoc


class AnnotationKind_Link extends AnnotationKind {
  const AnnotationKind_Link({required this.url, required this.title}): super._();
  

/// The URL from the `href` attribute, copied verbatim from the source HTML.
///
/// No URL decoding or normalization is performed: percent-encoded sequences, relative
/// paths, and protocol-relative URLs (`//example.com`) are all preserved exactly as
/// written in the source. Callers that need an absolute URL must resolve it against the
/// document base URL themselves.
 final  String url;
/// The `title` attribute of the `<a>` element, if present.
///
/// `None` when the `<a>` tag has no `title="..."` attribute. When present, the value
/// is copied verbatim — HTML entities within the title are not decoded.
 final  String title;

/// Create a copy of AnnotationKind
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$AnnotationKind_LinkCopyWith<AnnotationKind_Link> get copyWith => _$AnnotationKind_LinkCopyWithImpl<AnnotationKind_Link>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is AnnotationKind_Link&&(identical(other.url, url) || other.url == url)&&(identical(other.title, title) || other.title == title));
}


@override
int get hashCode => Object.hash(runtimeType,url,title);

@override
String toString() {
  return 'AnnotationKind.link(url: $url, title: $title)';
}


}

/// @nodoc
abstract mixin class $AnnotationKind_LinkCopyWith<$Res> implements $AnnotationKindCopyWith<$Res> {
  factory $AnnotationKind_LinkCopyWith(AnnotationKind_Link value, $Res Function(AnnotationKind_Link) _then) = _$AnnotationKind_LinkCopyWithImpl;
@useResult
$Res call({
 String url, String title
});




}
/// @nodoc
class _$AnnotationKind_LinkCopyWithImpl<$Res>
    implements $AnnotationKind_LinkCopyWith<$Res> {
  _$AnnotationKind_LinkCopyWithImpl(this._self, this._then);

  final AnnotationKind_Link _self;
  final $Res Function(AnnotationKind_Link) _then;

/// Create a copy of AnnotationKind
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? url = null,Object? title = null,}) {
  return _then(AnnotationKind_Link(
url: null == url ? _self.url : url // ignore: cast_nullable_to_non_nullable
as String,title: null == title ? _self.title : title // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc
mixin _$ConversionError {

 String get field0;
/// Create a copy of ConversionError
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$ConversionErrorCopyWith<ConversionError> get copyWith => _$ConversionErrorCopyWithImpl<ConversionError>(this as ConversionError, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is ConversionError&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode => Object.hash(runtimeType,field0);

@override
String toString() {
  return 'ConversionError(field0: $field0)';
}


}

/// @nodoc
abstract mixin class $ConversionErrorCopyWith<$Res>  {
  factory $ConversionErrorCopyWith(ConversionError value, $Res Function(ConversionError) _then) = _$ConversionErrorCopyWithImpl;
@useResult
$Res call({
 String field0
});




}
/// @nodoc
class _$ConversionErrorCopyWithImpl<$Res>
    implements $ConversionErrorCopyWith<$Res> {
  _$ConversionErrorCopyWithImpl(this._self, this._then);

  final ConversionError _self;
  final $Res Function(ConversionError) _then;

/// Create a copy of ConversionError
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? field0 = null,}) {
  return _then(_self.copyWith(
field0: null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as String,
  ));
}

}


/// Adds pattern-matching-related methods to [ConversionError].
extension ConversionErrorPatterns on ConversionError {
/// A variant of `map` that fallback to returning `orElse`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( ConversionError_ParseError value)?  parseError,TResult Function( ConversionError_SanitizationError value)?  sanitizationError,TResult Function( ConversionError_ConfigError value)?  configError,TResult Function( ConversionError_IoError value)?  ioError,TResult Function( ConversionError_Panic value)?  panic,TResult Function( ConversionError_InvalidInput value)?  invalidInput,TResult Function( ConversionError_Other value)?  other,required TResult orElse(),}){
final _that = this;
switch (_that) {
case ConversionError_ParseError() when parseError != null:
return parseError(_that);case ConversionError_SanitizationError() when sanitizationError != null:
return sanitizationError(_that);case ConversionError_ConfigError() when configError != null:
return configError(_that);case ConversionError_IoError() when ioError != null:
return ioError(_that);case ConversionError_Panic() when panic != null:
return panic(_that);case ConversionError_InvalidInput() when invalidInput != null:
return invalidInput(_that);case ConversionError_Other() when other != null:
return other(_that);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// Callbacks receives the raw object, upcasted.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case final Subclass2 value:
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( ConversionError_ParseError value)  parseError,required TResult Function( ConversionError_SanitizationError value)  sanitizationError,required TResult Function( ConversionError_ConfigError value)  configError,required TResult Function( ConversionError_IoError value)  ioError,required TResult Function( ConversionError_Panic value)  panic,required TResult Function( ConversionError_InvalidInput value)  invalidInput,required TResult Function( ConversionError_Other value)  other,}){
final _that = this;
switch (_that) {
case ConversionError_ParseError():
return parseError(_that);case ConversionError_SanitizationError():
return sanitizationError(_that);case ConversionError_ConfigError():
return configError(_that);case ConversionError_IoError():
return ioError(_that);case ConversionError_Panic():
return panic(_that);case ConversionError_InvalidInput():
return invalidInput(_that);case ConversionError_Other():
return other(_that);}
}
/// A variant of `map` that fallback to returning `null`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( ConversionError_ParseError value)?  parseError,TResult? Function( ConversionError_SanitizationError value)?  sanitizationError,TResult? Function( ConversionError_ConfigError value)?  configError,TResult? Function( ConversionError_IoError value)?  ioError,TResult? Function( ConversionError_Panic value)?  panic,TResult? Function( ConversionError_InvalidInput value)?  invalidInput,TResult? Function( ConversionError_Other value)?  other,}){
final _that = this;
switch (_that) {
case ConversionError_ParseError() when parseError != null:
return parseError(_that);case ConversionError_SanitizationError() when sanitizationError != null:
return sanitizationError(_that);case ConversionError_ConfigError() when configError != null:
return configError(_that);case ConversionError_IoError() when ioError != null:
return ioError(_that);case ConversionError_Panic() when panic != null:
return panic(_that);case ConversionError_InvalidInput() when invalidInput != null:
return invalidInput(_that);case ConversionError_Other() when other != null:
return other(_that);case _:
  return null;

}
}
/// A variant of `when` that fallback to an `orElse` callback.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function( String field0)?  parseError,TResult Function( String field0)?  sanitizationError,TResult Function( String field0)?  configError,TResult Function( String field0)?  ioError,TResult Function( String field0)?  panic,TResult Function( String field0)?  invalidInput,TResult Function( String field0)?  other,required TResult orElse(),}) {final _that = this;
switch (_that) {
case ConversionError_ParseError() when parseError != null:
return parseError(_that.field0);case ConversionError_SanitizationError() when sanitizationError != null:
return sanitizationError(_that.field0);case ConversionError_ConfigError() when configError != null:
return configError(_that.field0);case ConversionError_IoError() when ioError != null:
return ioError(_that.field0);case ConversionError_Panic() when panic != null:
return panic(_that.field0);case ConversionError_InvalidInput() when invalidInput != null:
return invalidInput(_that.field0);case ConversionError_Other() when other != null:
return other(_that.field0);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// As opposed to `map`, this offers destructuring.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case Subclass2(:final field2):
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function( String field0)  parseError,required TResult Function( String field0)  sanitizationError,required TResult Function( String field0)  configError,required TResult Function( String field0)  ioError,required TResult Function( String field0)  panic,required TResult Function( String field0)  invalidInput,required TResult Function( String field0)  other,}) {final _that = this;
switch (_that) {
case ConversionError_ParseError():
return parseError(_that.field0);case ConversionError_SanitizationError():
return sanitizationError(_that.field0);case ConversionError_ConfigError():
return configError(_that.field0);case ConversionError_IoError():
return ioError(_that.field0);case ConversionError_Panic():
return panic(_that.field0);case ConversionError_InvalidInput():
return invalidInput(_that.field0);case ConversionError_Other():
return other(_that.field0);}
}
/// A variant of `when` that fallback to returning `null`
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function( String field0)?  parseError,TResult? Function( String field0)?  sanitizationError,TResult? Function( String field0)?  configError,TResult? Function( String field0)?  ioError,TResult? Function( String field0)?  panic,TResult? Function( String field0)?  invalidInput,TResult? Function( String field0)?  other,}) {final _that = this;
switch (_that) {
case ConversionError_ParseError() when parseError != null:
return parseError(_that.field0);case ConversionError_SanitizationError() when sanitizationError != null:
return sanitizationError(_that.field0);case ConversionError_ConfigError() when configError != null:
return configError(_that.field0);case ConversionError_IoError() when ioError != null:
return ioError(_that.field0);case ConversionError_Panic() when panic != null:
return panic(_that.field0);case ConversionError_InvalidInput() when invalidInput != null:
return invalidInput(_that.field0);case ConversionError_Other() when other != null:
return other(_that.field0);case _:
  return null;

}
}

}

/// @nodoc


class ConversionError_ParseError extends ConversionError {
  const ConversionError_ParseError({required this.field0}): super._();
  

@override final  String field0;

/// Create a copy of ConversionError
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$ConversionError_ParseErrorCopyWith<ConversionError_ParseError> get copyWith => _$ConversionError_ParseErrorCopyWithImpl<ConversionError_ParseError>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is ConversionError_ParseError&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode => Object.hash(runtimeType,field0);

@override
String toString() {
  return 'ConversionError.parseError(field0: $field0)';
}


}

/// @nodoc
abstract mixin class $ConversionError_ParseErrorCopyWith<$Res> implements $ConversionErrorCopyWith<$Res> {
  factory $ConversionError_ParseErrorCopyWith(ConversionError_ParseError value, $Res Function(ConversionError_ParseError) _then) = _$ConversionError_ParseErrorCopyWithImpl;
@override @useResult
$Res call({
 String field0
});




}
/// @nodoc
class _$ConversionError_ParseErrorCopyWithImpl<$Res>
    implements $ConversionError_ParseErrorCopyWith<$Res> {
  _$ConversionError_ParseErrorCopyWithImpl(this._self, this._then);

  final ConversionError_ParseError _self;
  final $Res Function(ConversionError_ParseError) _then;

/// Create a copy of ConversionError
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? field0 = null,}) {
  return _then(ConversionError_ParseError(
field0: null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class ConversionError_SanitizationError extends ConversionError {
  const ConversionError_SanitizationError({required this.field0}): super._();
  

@override final  String field0;

/// Create a copy of ConversionError
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$ConversionError_SanitizationErrorCopyWith<ConversionError_SanitizationError> get copyWith => _$ConversionError_SanitizationErrorCopyWithImpl<ConversionError_SanitizationError>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is ConversionError_SanitizationError&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode => Object.hash(runtimeType,field0);

@override
String toString() {
  return 'ConversionError.sanitizationError(field0: $field0)';
}


}

/// @nodoc
abstract mixin class $ConversionError_SanitizationErrorCopyWith<$Res> implements $ConversionErrorCopyWith<$Res> {
  factory $ConversionError_SanitizationErrorCopyWith(ConversionError_SanitizationError value, $Res Function(ConversionError_SanitizationError) _then) = _$ConversionError_SanitizationErrorCopyWithImpl;
@override @useResult
$Res call({
 String field0
});




}
/// @nodoc
class _$ConversionError_SanitizationErrorCopyWithImpl<$Res>
    implements $ConversionError_SanitizationErrorCopyWith<$Res> {
  _$ConversionError_SanitizationErrorCopyWithImpl(this._self, this._then);

  final ConversionError_SanitizationError _self;
  final $Res Function(ConversionError_SanitizationError) _then;

/// Create a copy of ConversionError
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? field0 = null,}) {
  return _then(ConversionError_SanitizationError(
field0: null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class ConversionError_ConfigError extends ConversionError {
  const ConversionError_ConfigError({required this.field0}): super._();
  

@override final  String field0;

/// Create a copy of ConversionError
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$ConversionError_ConfigErrorCopyWith<ConversionError_ConfigError> get copyWith => _$ConversionError_ConfigErrorCopyWithImpl<ConversionError_ConfigError>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is ConversionError_ConfigError&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode => Object.hash(runtimeType,field0);

@override
String toString() {
  return 'ConversionError.configError(field0: $field0)';
}


}

/// @nodoc
abstract mixin class $ConversionError_ConfigErrorCopyWith<$Res> implements $ConversionErrorCopyWith<$Res> {
  factory $ConversionError_ConfigErrorCopyWith(ConversionError_ConfigError value, $Res Function(ConversionError_ConfigError) _then) = _$ConversionError_ConfigErrorCopyWithImpl;
@override @useResult
$Res call({
 String field0
});




}
/// @nodoc
class _$ConversionError_ConfigErrorCopyWithImpl<$Res>
    implements $ConversionError_ConfigErrorCopyWith<$Res> {
  _$ConversionError_ConfigErrorCopyWithImpl(this._self, this._then);

  final ConversionError_ConfigError _self;
  final $Res Function(ConversionError_ConfigError) _then;

/// Create a copy of ConversionError
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? field0 = null,}) {
  return _then(ConversionError_ConfigError(
field0: null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class ConversionError_IoError extends ConversionError {
  const ConversionError_IoError({required this.field0}): super._();
  

@override final  String field0;

/// Create a copy of ConversionError
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$ConversionError_IoErrorCopyWith<ConversionError_IoError> get copyWith => _$ConversionError_IoErrorCopyWithImpl<ConversionError_IoError>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is ConversionError_IoError&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode => Object.hash(runtimeType,field0);

@override
String toString() {
  return 'ConversionError.ioError(field0: $field0)';
}


}

/// @nodoc
abstract mixin class $ConversionError_IoErrorCopyWith<$Res> implements $ConversionErrorCopyWith<$Res> {
  factory $ConversionError_IoErrorCopyWith(ConversionError_IoError value, $Res Function(ConversionError_IoError) _then) = _$ConversionError_IoErrorCopyWithImpl;
@override @useResult
$Res call({
 String field0
});




}
/// @nodoc
class _$ConversionError_IoErrorCopyWithImpl<$Res>
    implements $ConversionError_IoErrorCopyWith<$Res> {
  _$ConversionError_IoErrorCopyWithImpl(this._self, this._then);

  final ConversionError_IoError _self;
  final $Res Function(ConversionError_IoError) _then;

/// Create a copy of ConversionError
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? field0 = null,}) {
  return _then(ConversionError_IoError(
field0: null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class ConversionError_Panic extends ConversionError {
  const ConversionError_Panic({required this.field0}): super._();
  

@override final  String field0;

/// Create a copy of ConversionError
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$ConversionError_PanicCopyWith<ConversionError_Panic> get copyWith => _$ConversionError_PanicCopyWithImpl<ConversionError_Panic>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is ConversionError_Panic&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode => Object.hash(runtimeType,field0);

@override
String toString() {
  return 'ConversionError.panic(field0: $field0)';
}


}

/// @nodoc
abstract mixin class $ConversionError_PanicCopyWith<$Res> implements $ConversionErrorCopyWith<$Res> {
  factory $ConversionError_PanicCopyWith(ConversionError_Panic value, $Res Function(ConversionError_Panic) _then) = _$ConversionError_PanicCopyWithImpl;
@override @useResult
$Res call({
 String field0
});




}
/// @nodoc
class _$ConversionError_PanicCopyWithImpl<$Res>
    implements $ConversionError_PanicCopyWith<$Res> {
  _$ConversionError_PanicCopyWithImpl(this._self, this._then);

  final ConversionError_Panic _self;
  final $Res Function(ConversionError_Panic) _then;

/// Create a copy of ConversionError
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? field0 = null,}) {
  return _then(ConversionError_Panic(
field0: null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class ConversionError_InvalidInput extends ConversionError {
  const ConversionError_InvalidInput({required this.field0}): super._();
  

@override final  String field0;

/// Create a copy of ConversionError
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$ConversionError_InvalidInputCopyWith<ConversionError_InvalidInput> get copyWith => _$ConversionError_InvalidInputCopyWithImpl<ConversionError_InvalidInput>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is ConversionError_InvalidInput&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode => Object.hash(runtimeType,field0);

@override
String toString() {
  return 'ConversionError.invalidInput(field0: $field0)';
}


}

/// @nodoc
abstract mixin class $ConversionError_InvalidInputCopyWith<$Res> implements $ConversionErrorCopyWith<$Res> {
  factory $ConversionError_InvalidInputCopyWith(ConversionError_InvalidInput value, $Res Function(ConversionError_InvalidInput) _then) = _$ConversionError_InvalidInputCopyWithImpl;
@override @useResult
$Res call({
 String field0
});




}
/// @nodoc
class _$ConversionError_InvalidInputCopyWithImpl<$Res>
    implements $ConversionError_InvalidInputCopyWith<$Res> {
  _$ConversionError_InvalidInputCopyWithImpl(this._self, this._then);

  final ConversionError_InvalidInput _self;
  final $Res Function(ConversionError_InvalidInput) _then;

/// Create a copy of ConversionError
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? field0 = null,}) {
  return _then(ConversionError_InvalidInput(
field0: null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class ConversionError_Other extends ConversionError {
  const ConversionError_Other({required this.field0}): super._();
  

@override final  String field0;

/// Create a copy of ConversionError
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$ConversionError_OtherCopyWith<ConversionError_Other> get copyWith => _$ConversionError_OtherCopyWithImpl<ConversionError_Other>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is ConversionError_Other&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode => Object.hash(runtimeType,field0);

@override
String toString() {
  return 'ConversionError.other(field0: $field0)';
}


}

/// @nodoc
abstract mixin class $ConversionError_OtherCopyWith<$Res> implements $ConversionErrorCopyWith<$Res> {
  factory $ConversionError_OtherCopyWith(ConversionError_Other value, $Res Function(ConversionError_Other) _then) = _$ConversionError_OtherCopyWithImpl;
@override @useResult
$Res call({
 String field0
});




}
/// @nodoc
class _$ConversionError_OtherCopyWithImpl<$Res>
    implements $ConversionError_OtherCopyWith<$Res> {
  _$ConversionError_OtherCopyWithImpl(this._self, this._then);

  final ConversionError_Other _self;
  final $Res Function(ConversionError_Other) _then;

/// Create a copy of ConversionError
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? field0 = null,}) {
  return _then(ConversionError_Other(
field0: null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc
mixin _$NodeContent {





@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is NodeContent);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'NodeContent()';
}


}

/// @nodoc
class $NodeContentCopyWith<$Res>  {
$NodeContentCopyWith(NodeContent _, $Res Function(NodeContent) __);
}


/// Adds pattern-matching-related methods to [NodeContent].
extension NodeContentPatterns on NodeContent {
/// A variant of `map` that fallback to returning `orElse`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( NodeContent_Heading value)?  heading,TResult Function( NodeContent_Paragraph value)?  paragraph,TResult Function( NodeContent_List value)?  list,TResult Function( NodeContent_ListItem value)?  listItem,TResult Function( NodeContent_Table value)?  table,TResult Function( NodeContent_Image value)?  image,TResult Function( NodeContent_Code value)?  code,TResult Function( NodeContent_Quote value)?  quote,TResult Function( NodeContent_DefinitionList value)?  definitionList,TResult Function( NodeContent_DefinitionItem value)?  definitionItem,TResult Function( NodeContent_RawBlock value)?  rawBlock,TResult Function( NodeContent_MetadataBlock value)?  metadataBlock,TResult Function( NodeContent_Group value)?  group,required TResult orElse(),}){
final _that = this;
switch (_that) {
case NodeContent_Heading() when heading != null:
return heading(_that);case NodeContent_Paragraph() when paragraph != null:
return paragraph(_that);case NodeContent_List() when list != null:
return list(_that);case NodeContent_ListItem() when listItem != null:
return listItem(_that);case NodeContent_Table() when table != null:
return table(_that);case NodeContent_Image() when image != null:
return image(_that);case NodeContent_Code() when code != null:
return code(_that);case NodeContent_Quote() when quote != null:
return quote(_that);case NodeContent_DefinitionList() when definitionList != null:
return definitionList(_that);case NodeContent_DefinitionItem() when definitionItem != null:
return definitionItem(_that);case NodeContent_RawBlock() when rawBlock != null:
return rawBlock(_that);case NodeContent_MetadataBlock() when metadataBlock != null:
return metadataBlock(_that);case NodeContent_Group() when group != null:
return group(_that);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// Callbacks receives the raw object, upcasted.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case final Subclass2 value:
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( NodeContent_Heading value)  heading,required TResult Function( NodeContent_Paragraph value)  paragraph,required TResult Function( NodeContent_List value)  list,required TResult Function( NodeContent_ListItem value)  listItem,required TResult Function( NodeContent_Table value)  table,required TResult Function( NodeContent_Image value)  image,required TResult Function( NodeContent_Code value)  code,required TResult Function( NodeContent_Quote value)  quote,required TResult Function( NodeContent_DefinitionList value)  definitionList,required TResult Function( NodeContent_DefinitionItem value)  definitionItem,required TResult Function( NodeContent_RawBlock value)  rawBlock,required TResult Function( NodeContent_MetadataBlock value)  metadataBlock,required TResult Function( NodeContent_Group value)  group,}){
final _that = this;
switch (_that) {
case NodeContent_Heading():
return heading(_that);case NodeContent_Paragraph():
return paragraph(_that);case NodeContent_List():
return list(_that);case NodeContent_ListItem():
return listItem(_that);case NodeContent_Table():
return table(_that);case NodeContent_Image():
return image(_that);case NodeContent_Code():
return code(_that);case NodeContent_Quote():
return quote(_that);case NodeContent_DefinitionList():
return definitionList(_that);case NodeContent_DefinitionItem():
return definitionItem(_that);case NodeContent_RawBlock():
return rawBlock(_that);case NodeContent_MetadataBlock():
return metadataBlock(_that);case NodeContent_Group():
return group(_that);}
}
/// A variant of `map` that fallback to returning `null`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( NodeContent_Heading value)?  heading,TResult? Function( NodeContent_Paragraph value)?  paragraph,TResult? Function( NodeContent_List value)?  list,TResult? Function( NodeContent_ListItem value)?  listItem,TResult? Function( NodeContent_Table value)?  table,TResult? Function( NodeContent_Image value)?  image,TResult? Function( NodeContent_Code value)?  code,TResult? Function( NodeContent_Quote value)?  quote,TResult? Function( NodeContent_DefinitionList value)?  definitionList,TResult? Function( NodeContent_DefinitionItem value)?  definitionItem,TResult? Function( NodeContent_RawBlock value)?  rawBlock,TResult? Function( NodeContent_MetadataBlock value)?  metadataBlock,TResult? Function( NodeContent_Group value)?  group,}){
final _that = this;
switch (_that) {
case NodeContent_Heading() when heading != null:
return heading(_that);case NodeContent_Paragraph() when paragraph != null:
return paragraph(_that);case NodeContent_List() when list != null:
return list(_that);case NodeContent_ListItem() when listItem != null:
return listItem(_that);case NodeContent_Table() when table != null:
return table(_that);case NodeContent_Image() when image != null:
return image(_that);case NodeContent_Code() when code != null:
return code(_that);case NodeContent_Quote() when quote != null:
return quote(_that);case NodeContent_DefinitionList() when definitionList != null:
return definitionList(_that);case NodeContent_DefinitionItem() when definitionItem != null:
return definitionItem(_that);case NodeContent_RawBlock() when rawBlock != null:
return rawBlock(_that);case NodeContent_MetadataBlock() when metadataBlock != null:
return metadataBlock(_that);case NodeContent_Group() when group != null:
return group(_that);case _:
  return null;

}
}
/// A variant of `when` that fallback to an `orElse` callback.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function( PlatformInt64 level,  String text)?  heading,TResult Function( String text)?  paragraph,TResult Function( bool ordered)?  list,TResult Function( String text)?  listItem,TResult Function( TableGrid grid)?  table,TResult Function( String description,  String src,  PlatformInt64 imageIndex)?  image,TResult Function( String text,  String language)?  code,TResult Function()?  quote,TResult Function()?  definitionList,TResult Function( String term,  String definition)?  definitionItem,TResult Function( String format,  String content)?  rawBlock,TResult Function( List<MetadataEntry> entries)?  metadataBlock,TResult Function( String label,  PlatformInt64 headingLevel,  String headingText)?  group,required TResult orElse(),}) {final _that = this;
switch (_that) {
case NodeContent_Heading() when heading != null:
return heading(_that.level,_that.text);case NodeContent_Paragraph() when paragraph != null:
return paragraph(_that.text);case NodeContent_List() when list != null:
return list(_that.ordered);case NodeContent_ListItem() when listItem != null:
return listItem(_that.text);case NodeContent_Table() when table != null:
return table(_that.grid);case NodeContent_Image() when image != null:
return image(_that.description,_that.src,_that.imageIndex);case NodeContent_Code() when code != null:
return code(_that.text,_that.language);case NodeContent_Quote() when quote != null:
return quote();case NodeContent_DefinitionList() when definitionList != null:
return definitionList();case NodeContent_DefinitionItem() when definitionItem != null:
return definitionItem(_that.term,_that.definition);case NodeContent_RawBlock() when rawBlock != null:
return rawBlock(_that.format,_that.content);case NodeContent_MetadataBlock() when metadataBlock != null:
return metadataBlock(_that.entries);case NodeContent_Group() when group != null:
return group(_that.label,_that.headingLevel,_that.headingText);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// As opposed to `map`, this offers destructuring.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case Subclass2(:final field2):
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function( PlatformInt64 level,  String text)  heading,required TResult Function( String text)  paragraph,required TResult Function( bool ordered)  list,required TResult Function( String text)  listItem,required TResult Function( TableGrid grid)  table,required TResult Function( String description,  String src,  PlatformInt64 imageIndex)  image,required TResult Function( String text,  String language)  code,required TResult Function()  quote,required TResult Function()  definitionList,required TResult Function( String term,  String definition)  definitionItem,required TResult Function( String format,  String content)  rawBlock,required TResult Function( List<MetadataEntry> entries)  metadataBlock,required TResult Function( String label,  PlatformInt64 headingLevel,  String headingText)  group,}) {final _that = this;
switch (_that) {
case NodeContent_Heading():
return heading(_that.level,_that.text);case NodeContent_Paragraph():
return paragraph(_that.text);case NodeContent_List():
return list(_that.ordered);case NodeContent_ListItem():
return listItem(_that.text);case NodeContent_Table():
return table(_that.grid);case NodeContent_Image():
return image(_that.description,_that.src,_that.imageIndex);case NodeContent_Code():
return code(_that.text,_that.language);case NodeContent_Quote():
return quote();case NodeContent_DefinitionList():
return definitionList();case NodeContent_DefinitionItem():
return definitionItem(_that.term,_that.definition);case NodeContent_RawBlock():
return rawBlock(_that.format,_that.content);case NodeContent_MetadataBlock():
return metadataBlock(_that.entries);case NodeContent_Group():
return group(_that.label,_that.headingLevel,_that.headingText);}
}
/// A variant of `when` that fallback to returning `null`
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function( PlatformInt64 level,  String text)?  heading,TResult? Function( String text)?  paragraph,TResult? Function( bool ordered)?  list,TResult? Function( String text)?  listItem,TResult? Function( TableGrid grid)?  table,TResult? Function( String description,  String src,  PlatformInt64 imageIndex)?  image,TResult? Function( String text,  String language)?  code,TResult? Function()?  quote,TResult? Function()?  definitionList,TResult? Function( String term,  String definition)?  definitionItem,TResult? Function( String format,  String content)?  rawBlock,TResult? Function( List<MetadataEntry> entries)?  metadataBlock,TResult? Function( String label,  PlatformInt64 headingLevel,  String headingText)?  group,}) {final _that = this;
switch (_that) {
case NodeContent_Heading() when heading != null:
return heading(_that.level,_that.text);case NodeContent_Paragraph() when paragraph != null:
return paragraph(_that.text);case NodeContent_List() when list != null:
return list(_that.ordered);case NodeContent_ListItem() when listItem != null:
return listItem(_that.text);case NodeContent_Table() when table != null:
return table(_that.grid);case NodeContent_Image() when image != null:
return image(_that.description,_that.src,_that.imageIndex);case NodeContent_Code() when code != null:
return code(_that.text,_that.language);case NodeContent_Quote() when quote != null:
return quote();case NodeContent_DefinitionList() when definitionList != null:
return definitionList();case NodeContent_DefinitionItem() when definitionItem != null:
return definitionItem(_that.term,_that.definition);case NodeContent_RawBlock() when rawBlock != null:
return rawBlock(_that.format,_that.content);case NodeContent_MetadataBlock() when metadataBlock != null:
return metadataBlock(_that.entries);case NodeContent_Group() when group != null:
return group(_that.label,_that.headingLevel,_that.headingText);case _:
  return null;

}
}

}

/// @nodoc


class NodeContent_Heading extends NodeContent {
  const NodeContent_Heading({required this.level, required this.text}): super._();
  

/// Heading level (1-6).
 final  PlatformInt64 level;
/// The heading text content.
 final  String text;

/// Create a copy of NodeContent
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$NodeContent_HeadingCopyWith<NodeContent_Heading> get copyWith => _$NodeContent_HeadingCopyWithImpl<NodeContent_Heading>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is NodeContent_Heading&&(identical(other.level, level) || other.level == level)&&(identical(other.text, text) || other.text == text));
}


@override
int get hashCode => Object.hash(runtimeType,level,text);

@override
String toString() {
  return 'NodeContent.heading(level: $level, text: $text)';
}


}

/// @nodoc
abstract mixin class $NodeContent_HeadingCopyWith<$Res> implements $NodeContentCopyWith<$Res> {
  factory $NodeContent_HeadingCopyWith(NodeContent_Heading value, $Res Function(NodeContent_Heading) _then) = _$NodeContent_HeadingCopyWithImpl;
@useResult
$Res call({
 PlatformInt64 level, String text
});




}
/// @nodoc
class _$NodeContent_HeadingCopyWithImpl<$Res>
    implements $NodeContent_HeadingCopyWith<$Res> {
  _$NodeContent_HeadingCopyWithImpl(this._self, this._then);

  final NodeContent_Heading _self;
  final $Res Function(NodeContent_Heading) _then;

/// Create a copy of NodeContent
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? level = null,Object? text = null,}) {
  return _then(NodeContent_Heading(
level: null == level ? _self.level : level // ignore: cast_nullable_to_non_nullable
as PlatformInt64,text: null == text ? _self.text : text // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class NodeContent_Paragraph extends NodeContent {
  const NodeContent_Paragraph({required this.text}): super._();
  

/// The paragraph text content.
 final  String text;

/// Create a copy of NodeContent
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$NodeContent_ParagraphCopyWith<NodeContent_Paragraph> get copyWith => _$NodeContent_ParagraphCopyWithImpl<NodeContent_Paragraph>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is NodeContent_Paragraph&&(identical(other.text, text) || other.text == text));
}


@override
int get hashCode => Object.hash(runtimeType,text);

@override
String toString() {
  return 'NodeContent.paragraph(text: $text)';
}


}

/// @nodoc
abstract mixin class $NodeContent_ParagraphCopyWith<$Res> implements $NodeContentCopyWith<$Res> {
  factory $NodeContent_ParagraphCopyWith(NodeContent_Paragraph value, $Res Function(NodeContent_Paragraph) _then) = _$NodeContent_ParagraphCopyWithImpl;
@useResult
$Res call({
 String text
});




}
/// @nodoc
class _$NodeContent_ParagraphCopyWithImpl<$Res>
    implements $NodeContent_ParagraphCopyWith<$Res> {
  _$NodeContent_ParagraphCopyWithImpl(this._self, this._then);

  final NodeContent_Paragraph _self;
  final $Res Function(NodeContent_Paragraph) _then;

/// Create a copy of NodeContent
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? text = null,}) {
  return _then(NodeContent_Paragraph(
text: null == text ? _self.text : text // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class NodeContent_List extends NodeContent {
  const NodeContent_List({required this.ordered}): super._();
  

/// Whether this is an ordered list.
 final  bool ordered;

/// Create a copy of NodeContent
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$NodeContent_ListCopyWith<NodeContent_List> get copyWith => _$NodeContent_ListCopyWithImpl<NodeContent_List>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is NodeContent_List&&(identical(other.ordered, ordered) || other.ordered == ordered));
}


@override
int get hashCode => Object.hash(runtimeType,ordered);

@override
String toString() {
  return 'NodeContent.list(ordered: $ordered)';
}


}

/// @nodoc
abstract mixin class $NodeContent_ListCopyWith<$Res> implements $NodeContentCopyWith<$Res> {
  factory $NodeContent_ListCopyWith(NodeContent_List value, $Res Function(NodeContent_List) _then) = _$NodeContent_ListCopyWithImpl;
@useResult
$Res call({
 bool ordered
});




}
/// @nodoc
class _$NodeContent_ListCopyWithImpl<$Res>
    implements $NodeContent_ListCopyWith<$Res> {
  _$NodeContent_ListCopyWithImpl(this._self, this._then);

  final NodeContent_List _self;
  final $Res Function(NodeContent_List) _then;

/// Create a copy of NodeContent
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? ordered = null,}) {
  return _then(NodeContent_List(
ordered: null == ordered ? _self.ordered : ordered // ignore: cast_nullable_to_non_nullable
as bool,
  ));
}


}

/// @nodoc


class NodeContent_ListItem extends NodeContent {
  const NodeContent_ListItem({required this.text}): super._();
  

/// The list item text content.
 final  String text;

/// Create a copy of NodeContent
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$NodeContent_ListItemCopyWith<NodeContent_ListItem> get copyWith => _$NodeContent_ListItemCopyWithImpl<NodeContent_ListItem>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is NodeContent_ListItem&&(identical(other.text, text) || other.text == text));
}


@override
int get hashCode => Object.hash(runtimeType,text);

@override
String toString() {
  return 'NodeContent.listItem(text: $text)';
}


}

/// @nodoc
abstract mixin class $NodeContent_ListItemCopyWith<$Res> implements $NodeContentCopyWith<$Res> {
  factory $NodeContent_ListItemCopyWith(NodeContent_ListItem value, $Res Function(NodeContent_ListItem) _then) = _$NodeContent_ListItemCopyWithImpl;
@useResult
$Res call({
 String text
});




}
/// @nodoc
class _$NodeContent_ListItemCopyWithImpl<$Res>
    implements $NodeContent_ListItemCopyWith<$Res> {
  _$NodeContent_ListItemCopyWithImpl(this._self, this._then);

  final NodeContent_ListItem _self;
  final $Res Function(NodeContent_ListItem) _then;

/// Create a copy of NodeContent
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? text = null,}) {
  return _then(NodeContent_ListItem(
text: null == text ? _self.text : text // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class NodeContent_Table extends NodeContent {
  const NodeContent_Table({required this.grid}): super._();
  

/// The table grid structure.
 final  TableGrid grid;

/// Create a copy of NodeContent
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$NodeContent_TableCopyWith<NodeContent_Table> get copyWith => _$NodeContent_TableCopyWithImpl<NodeContent_Table>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is NodeContent_Table&&(identical(other.grid, grid) || other.grid == grid));
}


@override
int get hashCode => Object.hash(runtimeType,grid);

@override
String toString() {
  return 'NodeContent.table(grid: $grid)';
}


}

/// @nodoc
abstract mixin class $NodeContent_TableCopyWith<$Res> implements $NodeContentCopyWith<$Res> {
  factory $NodeContent_TableCopyWith(NodeContent_Table value, $Res Function(NodeContent_Table) _then) = _$NodeContent_TableCopyWithImpl;
@useResult
$Res call({
 TableGrid grid
});




}
/// @nodoc
class _$NodeContent_TableCopyWithImpl<$Res>
    implements $NodeContent_TableCopyWith<$Res> {
  _$NodeContent_TableCopyWithImpl(this._self, this._then);

  final NodeContent_Table _self;
  final $Res Function(NodeContent_Table) _then;

/// Create a copy of NodeContent
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? grid = null,}) {
  return _then(NodeContent_Table(
grid: null == grid ? _self.grid : grid // ignore: cast_nullable_to_non_nullable
as TableGrid,
  ));
}


}

/// @nodoc


class NodeContent_Image extends NodeContent {
  const NodeContent_Image({required this.description, required this.src, required this.imageIndex}): super._();
  

/// Alt text or caption.
 final  String description;
/// Image source URL.
 final  String src;
/// Index into `ConversionResult.images` when image extraction is enabled.
 final  PlatformInt64 imageIndex;

/// Create a copy of NodeContent
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$NodeContent_ImageCopyWith<NodeContent_Image> get copyWith => _$NodeContent_ImageCopyWithImpl<NodeContent_Image>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is NodeContent_Image&&(identical(other.description, description) || other.description == description)&&(identical(other.src, src) || other.src == src)&&(identical(other.imageIndex, imageIndex) || other.imageIndex == imageIndex));
}


@override
int get hashCode => Object.hash(runtimeType,description,src,imageIndex);

@override
String toString() {
  return 'NodeContent.image(description: $description, src: $src, imageIndex: $imageIndex)';
}


}

/// @nodoc
abstract mixin class $NodeContent_ImageCopyWith<$Res> implements $NodeContentCopyWith<$Res> {
  factory $NodeContent_ImageCopyWith(NodeContent_Image value, $Res Function(NodeContent_Image) _then) = _$NodeContent_ImageCopyWithImpl;
@useResult
$Res call({
 String description, String src, PlatformInt64 imageIndex
});




}
/// @nodoc
class _$NodeContent_ImageCopyWithImpl<$Res>
    implements $NodeContent_ImageCopyWith<$Res> {
  _$NodeContent_ImageCopyWithImpl(this._self, this._then);

  final NodeContent_Image _self;
  final $Res Function(NodeContent_Image) _then;

/// Create a copy of NodeContent
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? description = null,Object? src = null,Object? imageIndex = null,}) {
  return _then(NodeContent_Image(
description: null == description ? _self.description : description // ignore: cast_nullable_to_non_nullable
as String,src: null == src ? _self.src : src // ignore: cast_nullable_to_non_nullable
as String,imageIndex: null == imageIndex ? _self.imageIndex : imageIndex // ignore: cast_nullable_to_non_nullable
as PlatformInt64,
  ));
}


}

/// @nodoc


class NodeContent_Code extends NodeContent {
  const NodeContent_Code({required this.text, required this.language}): super._();
  

/// The code text content.
 final  String text;
/// Programming language (from class="language-*" or similar).
 final  String language;

/// Create a copy of NodeContent
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$NodeContent_CodeCopyWith<NodeContent_Code> get copyWith => _$NodeContent_CodeCopyWithImpl<NodeContent_Code>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is NodeContent_Code&&(identical(other.text, text) || other.text == text)&&(identical(other.language, language) || other.language == language));
}


@override
int get hashCode => Object.hash(runtimeType,text,language);

@override
String toString() {
  return 'NodeContent.code(text: $text, language: $language)';
}


}

/// @nodoc
abstract mixin class $NodeContent_CodeCopyWith<$Res> implements $NodeContentCopyWith<$Res> {
  factory $NodeContent_CodeCopyWith(NodeContent_Code value, $Res Function(NodeContent_Code) _then) = _$NodeContent_CodeCopyWithImpl;
@useResult
$Res call({
 String text, String language
});




}
/// @nodoc
class _$NodeContent_CodeCopyWithImpl<$Res>
    implements $NodeContent_CodeCopyWith<$Res> {
  _$NodeContent_CodeCopyWithImpl(this._self, this._then);

  final NodeContent_Code _self;
  final $Res Function(NodeContent_Code) _then;

/// Create a copy of NodeContent
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? text = null,Object? language = null,}) {
  return _then(NodeContent_Code(
text: null == text ? _self.text : text // ignore: cast_nullable_to_non_nullable
as String,language: null == language ? _self.language : language // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class NodeContent_Quote extends NodeContent {
  const NodeContent_Quote(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is NodeContent_Quote);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'NodeContent.quote()';
}


}




/// @nodoc


class NodeContent_DefinitionList extends NodeContent {
  const NodeContent_DefinitionList(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is NodeContent_DefinitionList);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'NodeContent.definitionList()';
}


}




/// @nodoc


class NodeContent_DefinitionItem extends NodeContent {
  const NodeContent_DefinitionItem({required this.term, required this.definition}): super._();
  

/// The term being defined.
 final  String term;
/// The definition text.
 final  String definition;

/// Create a copy of NodeContent
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$NodeContent_DefinitionItemCopyWith<NodeContent_DefinitionItem> get copyWith => _$NodeContent_DefinitionItemCopyWithImpl<NodeContent_DefinitionItem>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is NodeContent_DefinitionItem&&(identical(other.term, term) || other.term == term)&&(identical(other.definition, definition) || other.definition == definition));
}


@override
int get hashCode => Object.hash(runtimeType,term,definition);

@override
String toString() {
  return 'NodeContent.definitionItem(term: $term, definition: $definition)';
}


}

/// @nodoc
abstract mixin class $NodeContent_DefinitionItemCopyWith<$Res> implements $NodeContentCopyWith<$Res> {
  factory $NodeContent_DefinitionItemCopyWith(NodeContent_DefinitionItem value, $Res Function(NodeContent_DefinitionItem) _then) = _$NodeContent_DefinitionItemCopyWithImpl;
@useResult
$Res call({
 String term, String definition
});




}
/// @nodoc
class _$NodeContent_DefinitionItemCopyWithImpl<$Res>
    implements $NodeContent_DefinitionItemCopyWith<$Res> {
  _$NodeContent_DefinitionItemCopyWithImpl(this._self, this._then);

  final NodeContent_DefinitionItem _self;
  final $Res Function(NodeContent_DefinitionItem) _then;

/// Create a copy of NodeContent
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? term = null,Object? definition = null,}) {
  return _then(NodeContent_DefinitionItem(
term: null == term ? _self.term : term // ignore: cast_nullable_to_non_nullable
as String,definition: null == definition ? _self.definition : definition // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class NodeContent_RawBlock extends NodeContent {
  const NodeContent_RawBlock({required this.format, required this.content}): super._();
  

/// The format of the raw content (e.g. "html", "css", "javascript").
 final  String format;
/// The raw content.
 final  String content;

/// Create a copy of NodeContent
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$NodeContent_RawBlockCopyWith<NodeContent_RawBlock> get copyWith => _$NodeContent_RawBlockCopyWithImpl<NodeContent_RawBlock>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is NodeContent_RawBlock&&(identical(other.format, format) || other.format == format)&&(identical(other.content, content) || other.content == content));
}


@override
int get hashCode => Object.hash(runtimeType,format,content);

@override
String toString() {
  return 'NodeContent.rawBlock(format: $format, content: $content)';
}


}

/// @nodoc
abstract mixin class $NodeContent_RawBlockCopyWith<$Res> implements $NodeContentCopyWith<$Res> {
  factory $NodeContent_RawBlockCopyWith(NodeContent_RawBlock value, $Res Function(NodeContent_RawBlock) _then) = _$NodeContent_RawBlockCopyWithImpl;
@useResult
$Res call({
 String format, String content
});




}
/// @nodoc
class _$NodeContent_RawBlockCopyWithImpl<$Res>
    implements $NodeContent_RawBlockCopyWith<$Res> {
  _$NodeContent_RawBlockCopyWithImpl(this._self, this._then);

  final NodeContent_RawBlock _self;
  final $Res Function(NodeContent_RawBlock) _then;

/// Create a copy of NodeContent
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? format = null,Object? content = null,}) {
  return _then(NodeContent_RawBlock(
format: null == format ? _self.format : format // ignore: cast_nullable_to_non_nullable
as String,content: null == content ? _self.content : content // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class NodeContent_MetadataBlock extends NodeContent {
  const NodeContent_MetadataBlock({required final  List<MetadataEntry> entries}): _entries = entries,super._();
  

/// Key-value metadata pairs.
 final  List<MetadataEntry> _entries;
/// Key-value metadata pairs.
 List<MetadataEntry> get entries {
  if (_entries is EqualUnmodifiableListView) return _entries;
  // ignore: implicit_dynamic_type
  return EqualUnmodifiableListView(_entries);
}


/// Create a copy of NodeContent
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$NodeContent_MetadataBlockCopyWith<NodeContent_MetadataBlock> get copyWith => _$NodeContent_MetadataBlockCopyWithImpl<NodeContent_MetadataBlock>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is NodeContent_MetadataBlock&&const DeepCollectionEquality().equals(other._entries, _entries));
}


@override
int get hashCode => Object.hash(runtimeType,const DeepCollectionEquality().hash(_entries));

@override
String toString() {
  return 'NodeContent.metadataBlock(entries: $entries)';
}


}

/// @nodoc
abstract mixin class $NodeContent_MetadataBlockCopyWith<$Res> implements $NodeContentCopyWith<$Res> {
  factory $NodeContent_MetadataBlockCopyWith(NodeContent_MetadataBlock value, $Res Function(NodeContent_MetadataBlock) _then) = _$NodeContent_MetadataBlockCopyWithImpl;
@useResult
$Res call({
 List<MetadataEntry> entries
});




}
/// @nodoc
class _$NodeContent_MetadataBlockCopyWithImpl<$Res>
    implements $NodeContent_MetadataBlockCopyWith<$Res> {
  _$NodeContent_MetadataBlockCopyWithImpl(this._self, this._then);

  final NodeContent_MetadataBlock _self;
  final $Res Function(NodeContent_MetadataBlock) _then;

/// Create a copy of NodeContent
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? entries = null,}) {
  return _then(NodeContent_MetadataBlock(
entries: null == entries ? _self._entries : entries // ignore: cast_nullable_to_non_nullable
as List<MetadataEntry>,
  ));
}


}

/// @nodoc


class NodeContent_Group extends NodeContent {
  const NodeContent_Group({required this.label, required this.headingLevel, required this.headingText}): super._();
  

/// Optional section label.
 final  String label;
/// The heading level that created this group.
 final  PlatformInt64 headingLevel;
/// The heading text that created this group.
 final  String headingText;

/// Create a copy of NodeContent
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$NodeContent_GroupCopyWith<NodeContent_Group> get copyWith => _$NodeContent_GroupCopyWithImpl<NodeContent_Group>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is NodeContent_Group&&(identical(other.label, label) || other.label == label)&&(identical(other.headingLevel, headingLevel) || other.headingLevel == headingLevel)&&(identical(other.headingText, headingText) || other.headingText == headingText));
}


@override
int get hashCode => Object.hash(runtimeType,label,headingLevel,headingText);

@override
String toString() {
  return 'NodeContent.group(label: $label, headingLevel: $headingLevel, headingText: $headingText)';
}


}

/// @nodoc
abstract mixin class $NodeContent_GroupCopyWith<$Res> implements $NodeContentCopyWith<$Res> {
  factory $NodeContent_GroupCopyWith(NodeContent_Group value, $Res Function(NodeContent_Group) _then) = _$NodeContent_GroupCopyWithImpl;
@useResult
$Res call({
 String label, PlatformInt64 headingLevel, String headingText
});




}
/// @nodoc
class _$NodeContent_GroupCopyWithImpl<$Res>
    implements $NodeContent_GroupCopyWith<$Res> {
  _$NodeContent_GroupCopyWithImpl(this._self, this._then);

  final NodeContent_Group _self;
  final $Res Function(NodeContent_Group) _then;

/// Create a copy of NodeContent
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? label = null,Object? headingLevel = null,Object? headingText = null,}) {
  return _then(NodeContent_Group(
label: null == label ? _self.label : label // ignore: cast_nullable_to_non_nullable
as String,headingLevel: null == headingLevel ? _self.headingLevel : headingLevel // ignore: cast_nullable_to_non_nullable
as PlatformInt64,headingText: null == headingText ? _self.headingText : headingText // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc
mixin _$VisitResult {





@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is VisitResult);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'VisitResult()';
}


}

/// @nodoc
class $VisitResultCopyWith<$Res>  {
$VisitResultCopyWith(VisitResult _, $Res Function(VisitResult) __);
}


/// Adds pattern-matching-related methods to [VisitResult].
extension VisitResultPatterns on VisitResult {
/// A variant of `map` that fallback to returning `orElse`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( VisitResult_Continue value)?  continue_,TResult Function( VisitResult_Custom value)?  custom,TResult Function( VisitResult_Skip value)?  skip,TResult Function( VisitResult_PreserveHtml value)?  preserveHtml,TResult Function( VisitResult_Error value)?  error,required TResult orElse(),}){
final _that = this;
switch (_that) {
case VisitResult_Continue() when continue_ != null:
return continue_(_that);case VisitResult_Custom() when custom != null:
return custom(_that);case VisitResult_Skip() when skip != null:
return skip(_that);case VisitResult_PreserveHtml() when preserveHtml != null:
return preserveHtml(_that);case VisitResult_Error() when error != null:
return error(_that);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// Callbacks receives the raw object, upcasted.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case final Subclass2 value:
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( VisitResult_Continue value)  continue_,required TResult Function( VisitResult_Custom value)  custom,required TResult Function( VisitResult_Skip value)  skip,required TResult Function( VisitResult_PreserveHtml value)  preserveHtml,required TResult Function( VisitResult_Error value)  error,}){
final _that = this;
switch (_that) {
case VisitResult_Continue():
return continue_(_that);case VisitResult_Custom():
return custom(_that);case VisitResult_Skip():
return skip(_that);case VisitResult_PreserveHtml():
return preserveHtml(_that);case VisitResult_Error():
return error(_that);}
}
/// A variant of `map` that fallback to returning `null`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( VisitResult_Continue value)?  continue_,TResult? Function( VisitResult_Custom value)?  custom,TResult? Function( VisitResult_Skip value)?  skip,TResult? Function( VisitResult_PreserveHtml value)?  preserveHtml,TResult? Function( VisitResult_Error value)?  error,}){
final _that = this;
switch (_that) {
case VisitResult_Continue() when continue_ != null:
return continue_(_that);case VisitResult_Custom() when custom != null:
return custom(_that);case VisitResult_Skip() when skip != null:
return skip(_that);case VisitResult_PreserveHtml() when preserveHtml != null:
return preserveHtml(_that);case VisitResult_Error() when error != null:
return error(_that);case _:
  return null;

}
}
/// A variant of `when` that fallback to an `orElse` callback.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function()?  continue_,TResult Function( String field0)?  custom,TResult Function()?  skip,TResult Function()?  preserveHtml,TResult Function( String field0)?  error,required TResult orElse(),}) {final _that = this;
switch (_that) {
case VisitResult_Continue() when continue_ != null:
return continue_();case VisitResult_Custom() when custom != null:
return custom(_that.field0);case VisitResult_Skip() when skip != null:
return skip();case VisitResult_PreserveHtml() when preserveHtml != null:
return preserveHtml();case VisitResult_Error() when error != null:
return error(_that.field0);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// As opposed to `map`, this offers destructuring.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case Subclass2(:final field2):
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function()  continue_,required TResult Function( String field0)  custom,required TResult Function()  skip,required TResult Function()  preserveHtml,required TResult Function( String field0)  error,}) {final _that = this;
switch (_that) {
case VisitResult_Continue():
return continue_();case VisitResult_Custom():
return custom(_that.field0);case VisitResult_Skip():
return skip();case VisitResult_PreserveHtml():
return preserveHtml();case VisitResult_Error():
return error(_that.field0);}
}
/// A variant of `when` that fallback to returning `null`
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function()?  continue_,TResult? Function( String field0)?  custom,TResult? Function()?  skip,TResult? Function()?  preserveHtml,TResult? Function( String field0)?  error,}) {final _that = this;
switch (_that) {
case VisitResult_Continue() when continue_ != null:
return continue_();case VisitResult_Custom() when custom != null:
return custom(_that.field0);case VisitResult_Skip() when skip != null:
return skip();case VisitResult_PreserveHtml() when preserveHtml != null:
return preserveHtml();case VisitResult_Error() when error != null:
return error(_that.field0);case _:
  return null;

}
}

}

/// @nodoc


class VisitResult_Continue extends VisitResult {
  const VisitResult_Continue(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is VisitResult_Continue);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'VisitResult.continue_()';
}


}




/// @nodoc


class VisitResult_Custom extends VisitResult {
  const VisitResult_Custom({required this.field0}): super._();
  

 final  String field0;

/// Create a copy of VisitResult
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$VisitResult_CustomCopyWith<VisitResult_Custom> get copyWith => _$VisitResult_CustomCopyWithImpl<VisitResult_Custom>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is VisitResult_Custom&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode => Object.hash(runtimeType,field0);

@override
String toString() {
  return 'VisitResult.custom(field0: $field0)';
}


}

/// @nodoc
abstract mixin class $VisitResult_CustomCopyWith<$Res> implements $VisitResultCopyWith<$Res> {
  factory $VisitResult_CustomCopyWith(VisitResult_Custom value, $Res Function(VisitResult_Custom) _then) = _$VisitResult_CustomCopyWithImpl;
@useResult
$Res call({
 String field0
});




}
/// @nodoc
class _$VisitResult_CustomCopyWithImpl<$Res>
    implements $VisitResult_CustomCopyWith<$Res> {
  _$VisitResult_CustomCopyWithImpl(this._self, this._then);

  final VisitResult_Custom _self;
  final $Res Function(VisitResult_Custom) _then;

/// Create a copy of VisitResult
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? field0 = null,}) {
  return _then(VisitResult_Custom(
field0: null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class VisitResult_Skip extends VisitResult {
  const VisitResult_Skip(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is VisitResult_Skip);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'VisitResult.skip()';
}


}




/// @nodoc


class VisitResult_PreserveHtml extends VisitResult {
  const VisitResult_PreserveHtml(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is VisitResult_PreserveHtml);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'VisitResult.preserveHtml()';
}


}




/// @nodoc


class VisitResult_Error extends VisitResult {
  const VisitResult_Error({required this.field0}): super._();
  

 final  String field0;

/// Create a copy of VisitResult
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$VisitResult_ErrorCopyWith<VisitResult_Error> get copyWith => _$VisitResult_ErrorCopyWithImpl<VisitResult_Error>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is VisitResult_Error&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode => Object.hash(runtimeType,field0);

@override
String toString() {
  return 'VisitResult.error(field0: $field0)';
}


}

/// @nodoc
abstract mixin class $VisitResult_ErrorCopyWith<$Res> implements $VisitResultCopyWith<$Res> {
  factory $VisitResult_ErrorCopyWith(VisitResult_Error value, $Res Function(VisitResult_Error) _then) = _$VisitResult_ErrorCopyWithImpl;
@useResult
$Res call({
 String field0
});




}
/// @nodoc
class _$VisitResult_ErrorCopyWithImpl<$Res>
    implements $VisitResult_ErrorCopyWith<$Res> {
  _$VisitResult_ErrorCopyWithImpl(this._self, this._then);

  final VisitResult_Error _self;
  final $Res Function(VisitResult_Error) _then;

/// Create a copy of VisitResult
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? field0 = null,}) {
  return _then(VisitResult_Error(
field0: null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

// dart format on
