#import <Foundation/Foundation.h>
#import <Metal/Metal.h>
#import <QuartzCore/QuartzCore.h>

NS_ASSUME_NONNULL_BEGIN

BOOL ca_objc_set_pixel_format(CAMetalLayer *layer, MTLPixelFormat pixelFormat, NSString * _Nullable * _Nullable reason);
BOOL ca_objc_set_frame(CALayer *layer, CGRect frame, NSString * _Nullable * _Nullable reason);
BOOL ca_objc_set_bounds(CALayer *layer, CGRect bounds, NSString * _Nullable * _Nullable reason);
BOOL ca_objc_set_position(CALayer *layer, CGPoint position, NSString * _Nullable * _Nullable reason);

NS_ASSUME_NONNULL_END
