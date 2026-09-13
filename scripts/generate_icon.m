#import <Cocoa/Cocoa.h>
#import <CoreGraphics/CoreGraphics.h>

void draw_icon(CGContextRef ctx, CGFloat size) {
    CGFloat scale = size / 1024.0;
    
    // 1. Shadow for squircle
    CGContextSaveGState(ctx);
    CGColorRef shadowColor = CGColorCreateGenericRGB(0, 0, 0, 0.45);
    CGContextSetShadowWithColor(ctx, CGSizeMake(0, -18 * scale), 32 * scale, shadowColor);
    CGColorRelease(shadowColor);
    
    CGRect iconRect = CGRectMake(100 * scale, 100 * scale, 824 * scale, 824 * scale);
    CGPathRef squirclePath = CGPathCreateWithRoundedRect(iconRect, 185 * scale, 185 * scale, NULL);
    CGContextAddPath(ctx, squirclePath);
    CGContextSetRGBFillColor(ctx, 0.05, 0.07, 0.13, 1.0);
    CGContextFillPath(ctx);
    CGContextRestoreGState(ctx);
    
    // 2. Base Squircle with Gradient
    CGContextSaveGState(ctx);
    CGContextAddPath(ctx, squirclePath);
    CGContextClip(ctx);
    
    CGColorSpaceRef colorSpace = CGColorSpaceCreateDeviceRGB();
    
    // Gradient: Slate Dark Navy -> Deep Indigo Night
    CGFloat gradComponents[] = {
        0.08, 0.12, 0.22, 1.0,  // Slate
        0.04, 0.05, 0.14, 1.0   // Deep Night
    };
    CGFloat locations[] = {0.0, 1.0};
    CGGradientRef grad = CGGradientCreateWithColorComponents(colorSpace, gradComponents, locations, 2);
    CGContextDrawLinearGradient(ctx, grad, CGPointMake(100 * scale, 924 * scale), CGPointMake(924 * scale, 100 * scale), 0);
    CGGradientRelease(grad);
    
    // Radial glow
    CGFloat glowComponents[] = {
        0.12, 0.45, 0.95, 0.35,
        0.02, 0.05, 0.15, 0.0
    };
    CGGradientRef glow = CGGradientCreateWithColorComponents(colorSpace, glowComponents, locations, 2);
    CGContextDrawRadialGradient(ctx, glow, CGPointMake(512 * scale, 530 * scale), 0, CGPointMake(512 * scale, 530 * scale), 420 * scale, 0);
    CGGradientRelease(glow);
    
    // Top border rim highlight
    CGContextSetLineWidth(ctx, 3.0 * scale);
    CGContextSetRGBStrokeColor(ctx, 1.0, 1.0, 1.0, 0.18);
    CGContextAddPath(ctx, squirclePath);
    CGContextStrokePath(ctx);
    
    // 3. Central Gauge / Meter Arc
    CGPoint center = CGPointMake(512 * scale, 520 * scale);
    CGFloat radius = 230 * scale;
    
    // Background arc
    CGMutablePathRef bgArc = CGPathCreateMutable();
    CGPathAddArc(bgArc, NULL, center.x, center.y, radius, M_PI * 0.8, M_PI * 0.2, true);
    CGContextSaveGState(ctx);
    CGContextSetLineWidth(ctx, 22 * scale);
    CGContextSetLineCap(ctx, kCGLineCapRound);
    CGContextSetRGBStrokeColor(ctx, 0.15, 0.25, 0.42, 0.5);
    CGContextAddPath(ctx, bgArc);
    CGContextStrokePath(ctx);
    CGPathRelease(bgArc);
    
    // Glowing cyan-emerald active arc
    CGMutablePathRef activeArc = CGPathCreateMutable();
    CGPathAddArc(activeArc, NULL, center.x, center.y, radius, M_PI * 0.8, M_PI * 1.72, true);
    CGColorRef cyanGlow = CGColorCreateGenericRGB(0.0, 0.85, 1.0, 0.8);
    CGContextSetShadowWithColor(ctx, CGSizeZero, 18 * scale, cyanGlow);
    CGColorRelease(cyanGlow);
    CGContextSetRGBStrokeColor(ctx, 0.05, 0.88, 0.95, 1.0);
    CGContextAddPath(ctx, activeArc);
    CGContextStrokePath(ctx);
    CGPathRelease(activeArc);
    CGContextRestoreGState(ctx);
    
    // 4. Stylized Cloud Icon in center
    CGRect cRect = CGRectMake(320 * scale, 410 * scale, 384 * scale, 260 * scale);
    CGFloat y = cRect.origin.y + cRect.size.height * 0.25;
    CGFloat w = cRect.size.width;
    CGFloat h = cRect.size.height;
    
    CGMutablePathRef cloud = CGPathCreateMutable();
    CGPathAddEllipseInRect(cloud, NULL, CGRectMake(cRect.origin.x + w * 0.15, y, w * 0.38, h * 0.38));
    CGPathAddEllipseInRect(cloud, NULL, CGRectMake(cRect.origin.x + w * 0.32, y + h * 0.12, w * 0.44, h * 0.44));
    CGPathAddEllipseInRect(cloud, NULL, CGRectMake(cRect.origin.x + w * 0.55, y + h * 0.02, w * 0.32, h * 0.32));
    CGPathAddRoundedRect(cloud, NULL, CGRectMake(cRect.origin.x + w * 0.18, y, w * 0.65, h * 0.26), h * 0.13, h * 0.13);
    
    CGContextSaveGState(ctx);
    CGColorRef cloudGlow = CGColorCreateGenericRGB(0.0, 0.75, 1.0, 0.65);
    CGContextSetShadowWithColor(ctx, CGSizeMake(0, -8 * scale), 28 * scale, cloudGlow);
    CGColorRelease(cloudGlow);
    CGContextAddPath(ctx, cloud);
    CGContextClip(ctx);
    
    CGFloat cloudGradComponents[] = {
        0.96, 0.99, 1.0, 0.98,
        0.35, 0.78, 1.0, 0.90,
        0.10, 0.45, 0.95, 0.92
    };
    CGFloat cloudLocs[] = {0.0, 0.5, 1.0};
    CGGradientRef cloudGrad = CGGradientCreateWithColorComponents(colorSpace, cloudGradComponents, cloudLocs, 3);
    CGContextDrawLinearGradient(ctx, cloudGrad, CGPointMake(CGRectGetMidX(cRect), CGRectGetMaxY(cRect)), CGPointMake(CGRectGetMidX(cRect), CGRectGetMinY(cRect)), 0);
    CGGradientRelease(cloudGrad);
    CGContextRestoreGState(ctx);
    CGPathRelease(cloud);
    
    // 5. Activity Pulse Wave in Center (Glowing Green)
    CGMutablePathRef pulse = CGPathCreateMutable();
    CGPathMoveToPoint(pulse, NULL, 370 * scale, 475 * scale);
    CGPathAddLineToPoint(pulse, NULL, 440 * scale, 475 * scale);
    CGPathAddLineToPoint(pulse, NULL, 480 * scale, 535 * scale);
    CGPathAddLineToPoint(pulse, NULL, 525 * scale, 415 * scale);
    CGPathAddLineToPoint(pulse, NULL, 570 * scale, 505 * scale);
    CGPathAddLineToPoint(pulse, NULL, 600 * scale, 475 * scale);
    CGPathAddLineToPoint(pulse, NULL, 654 * scale, 475 * scale);
    
    CGContextSaveGState(ctx);
    CGContextSetLineWidth(ctx, 14 * scale);
    CGContextSetLineCap(ctx, kCGLineCapRound);
    CGContextSetLineJoin(ctx, kCGLineJoinRound);
    CGColorRef greenGlow = CGColorCreateGenericRGB(0.2, 1.0, 0.65, 0.95);
    CGContextSetShadowWithColor(ctx, CGSizeZero, 22 * scale, greenGlow);
    CGColorRelease(greenGlow);
    CGContextSetRGBStrokeColor(ctx, 0.15, 0.98, 0.65, 1.0);
    CGContextAddPath(ctx, pulse);
    CGContextStrokePath(ctx);
    CGContextRestoreGState(ctx);
    CGPathRelease(pulse);
    
    // 6. Text "CODEX"
    NSString *text = @"CODEX MONITOR";
    NSDictionary *attrs = @{
        NSFontAttributeName: [NSFont systemFontOfSize:52 * scale weight:NSFontWeightBlack],
        NSForegroundColorAttributeName: [NSColor colorWithRed:0.65 green:0.80 blue:0.98 alpha:0.88],
        NSKernAttributeName: @(7.0 * scale)
    };
    NSAttributedString *attrStr = [[NSAttributedString alloc] initWithString:text attributes:attrs];
    NSSize strSize = [attrStr size];
    NSRect strRect = NSMakeRect((size - strSize.width) / 2.0, 220 * scale, strSize.width, strSize.height);
    [attrStr drawInRect:strRect];
    
    CGContextRestoreGState(ctx); // squircle clip
    CGPathRelease(squirclePath);
    CGColorSpaceRelease(colorSpace);
}

void render_png(NSString *path, CGFloat size) {
    size_t s = (size_t)size;
    CGColorSpaceRef cs = CGColorSpaceCreateDeviceRGB();
    CGContextRef ctx = CGBitmapContextCreate(NULL, s, s, 8, s * 4, cs, kCGImageAlphaPremultipliedLast);
    CGColorSpaceRelease(cs);
    
    [NSGraphicsContext saveGraphicsState];
    [NSGraphicsContext setCurrentContext:[NSGraphicsContext graphicsContextWithCGContext:ctx flipped:NO]];
    
    draw_icon(ctx, size);
    
    [NSGraphicsContext restoreGraphicsState];
    
    CGImageRef image = CGBitmapContextCreateImage(ctx);
    CGContextRelease(ctx);
    
    CFURLRef url = (__bridge CFURLRef)[NSURL fileURLWithPath:path];
    CGImageDestinationRef dest = CGImageDestinationCreateWithURL(url, kUTTypePNG, 1, NULL);
    CGImageDestinationAddImage(dest, image, NULL);
    CGImageDestinationFinalize(dest);
    CFRelease(dest);
    CGImageRelease(image);
}

int main(int argc, const char * argv[]) {
    @autoreleasepool {
        if (argc >= 4 && strcmp(argv[1], "--set-icon") == 0) {
            NSString *iconPath = [NSString stringWithUTF8String:argv[2]];
            NSString *filePath = [NSString stringWithUTF8String:argv[3]];
            NSImage *img = [[NSImage alloc] initWithContentsOfFile:iconPath];
            BOOL ok = [[NSWorkspace sharedWorkspace] setIcon:img forFile:filePath options:0];
            NSLog(@"Set icon on %@: %@", filePath, ok ? @"SUCCESS" : @"FAILED");
            return ok ? 0 : 1;
        }

        NSString *assetsDir = @"assets";
        NSString *iconsetDir = [assetsDir stringByAppendingPathComponent:@"AppIcon.iconset"];
        [[NSFileManager defaultManager] createDirectoryAtPath:iconsetDir withIntermediateDirectories:YES attributes:nil error:nil];
        
        NSArray *sizes = @[
            @{@"name": @"icon_16x16.png", @"size": @(16)},
            @{@"name": @"icon_16x16@2x.png", @"size": @(32)},
            @{@"name": @"icon_32x32.png", @"size": @(32)},
            @{@"name": @"icon_32x32@2x.png", @"size": @(64)},
            @{@"name": @"icon_128x128.png", @"size": @(128)},
            @{@"name": @"icon_128x128@2x.png", @"size": @(256)},
            @{@"name": @"icon_256x256.png", @"size": @(256)},
            @{@"name": @"icon_256x256@2x.png", @"size": @(512)},
            @{@"name": @"icon_512x512.png", @"size": @(512)},
            @{@"name": @"icon_512x512@2x.png", @"size": @(1024)}
        ];
        
        for (NSDictionary *item in sizes) {
            NSString *p = [iconsetDir stringByAppendingPathComponent:item[@"name"]];
            CGFloat sz = [item[@"size"] doubleValue];
            render_png(p, sz);
        }
        
        // Also save master 1024
        render_png([assetsDir stringByAppendingPathComponent:@"icon_1024.png"], 1024);
        NSLog(@"Successfully generated all icon sizes in %@", iconsetDir);
    }
    return 0;
}
