#import "CoreAnimationObjCBridge.h"

static void ca_objc_store_reason(NSException *exception, NSString * _Nullable * _Nullable reason) {
    if (reason != NULL) {
        *reason = exception.reason ?: exception.name;
    }
}

BOOL ca_objc_set_pixel_format(CAMetalLayer *layer, MTLPixelFormat pixelFormat, NSString * _Nullable * _Nullable reason) {
    @try {
        layer.pixelFormat = pixelFormat;
        return YES;
    } @catch (NSException *exception) {
        ca_objc_store_reason(exception, reason);
        return NO;
    }
}

BOOL ca_objc_set_frame(CALayer *layer, CGRect frame, NSString * _Nullable * _Nullable reason) {
    @try {
        layer.frame = frame;
        return YES;
    } @catch (NSException *exception) {
        ca_objc_store_reason(exception, reason);
        return NO;
    }
}

BOOL ca_objc_set_bounds(CALayer *layer, CGRect bounds, NSString * _Nullable * _Nullable reason) {
    @try {
        layer.bounds = bounds;
        return YES;
    } @catch (NSException *exception) {
        ca_objc_store_reason(exception, reason);
        return NO;
    }
}

BOOL ca_objc_set_position(CALayer *layer, CGPoint position, NSString * _Nullable * _Nullable reason) {
    @try {
        layer.position = position;
        return YES;
    } @catch (NSException *exception) {
        ca_objc_store_reason(exception, reason);
        return NO;
    }
}
