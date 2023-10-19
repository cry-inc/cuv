// A Unit Vector to 16-bit word conversion algorithm
// based on work of Rafael Baptista (rafael@oroboro.com)
// Accuracy improved by O.D. (punkfloyd@rocketmail.com)
// Input vector does not have to be unit length.
// Source: https://web.archive.org/web/20201022211551/https://oroboro.com/compressed-unit-vectors/

#include <stdint.h>
#include <assert.h>
#include <math.h>
#include <stdio.h>

// upper 3 bits
#define SIGN_MASK 0xe000
#define XSIGN_MASK 0x8000
#define YSIGN_MASK 0x4000
#define ZSIGN_MASK 0x2000

// middle 6 bits - xbits
#define TOP_MASK 0x1f80

// lower 7 bits - ybits
#define BOTTOM_MASK 0x007f

static float lut[0x2000];

static void init()
{
    for (int idx = 0; idx < 0x2000; idx++)
    {
        long xbits = idx >> 7;
        long ybits = idx & BOTTOM_MASK;

        // map the numbers back to the triangle (0,0)-(0,127)-(127,0)
        if ((xbits + ybits) >= 127)
        {
            xbits = 127 - xbits;
            ybits = 127 - ybits;
        }

        // convert to 3D vectors
        float x = (float)xbits;
        float y = (float)ybits;
        float z = (float)(126 - xbits - ybits);

        // calculate the amount of normalization required
        lut[idx] = 1.0f / sqrtf(y * y + z * z + x * x);
        assert(isfinite(lut[idx]));
    }
}

uint16_t packVector(float x, float y, float z)
{
    uint16_t res = 0;

    if (x < 0)
    {
        res |= XSIGN_MASK;
        x = -x;
    }
    if (y < 0)
    {
        res |= YSIGN_MASK;
        y = -y;
    }
    if (z < 0)
    {
        res |= ZSIGN_MASK;
        z = -z;
    }

    // project the normal onto the plane that goes through
    // X0=(1,0,0),Y0=(0,1,0),Z0=(0,0,1).

    // on that plane we choose an (projective!) coordinate system
    // such that X0->(0,0), Y0->(126,0), Z0->(0,126),(0,0,0)->Infinity

    // a little slower... old pack was 4 multiplies and 2 adds.
    // This is 2 multiplies, 2 adds, and a divide....
    float w = 126.0f / (x + y + z);
    long xbits = (long)(x * w);
    long ybits = (long)(y * w);

    assert(xbits < 127);
    assert(xbits >= 0);
    assert(ybits < 127);
    assert(ybits >= 0);

    // Now we can be sure that 0<=xp<=126, 0<=yp<=126, 0<=xp+yp<=126
    // however for the sampling we want to transform this triangle
    // into a rectangle.
    if (xbits >= 64)
    {
        xbits = 127 - xbits;
        ybits = 127 - ybits;
    }

    // now we that have xp in the range (0,127) and yp in the
    // range (0,63), we can pack all the bits together
    res |= (xbits << 7);
    res |= ybits;

    return res;
}

void unpackVector(uint16_t cv, float *x, float *y, float *z)
{
    // if we do a straightforward backward transform
    // we will get points on the plane X0,Y0,Z0
    // however we need points on a sphere that goes through
    // these points.
    // therefore we need to adjust x,y,z so that x^2+y^2+z^2=1

    // by normalizing the vector. We have already precalculated
    // the amount by which we need to scale, so all we do is a
    // table lookup and a multiplication

    // get the x and y bits
    long xbits = ((cv & TOP_MASK) >> 7);
    long ybits = (cv & BOTTOM_MASK);

    // map the numbers back to the triangle (0,0)-(0,126)-(126,0)
    if ((xbits + ybits) >= 127)
    {
        xbits = 127 - xbits;
        ybits = 127 - ybits;
    }

    // do the inverse transform and normalization
    // costs 3 extra multiplies and 2 subtracts. No big deal.
    float uvadj = lut[cv & ~SIGN_MASK];
    *x = uvadj * (float)xbits;
    *y = uvadj * (float)ybits;
    *z = uvadj * (float)(126 - xbits - ybits);

    // set all the sign bits
    if (cv & XSIGN_MASK)
        *x = -*x;
    if (cv & YSIGN_MASK)
        *y = -*y;
    if (cv & ZSIGN_MASK)
        *z = -*z;
}

void main()
{
    init();
    uint16_t cv = packVector(1.0f, 0.0f, 0.0f);
    printf("Compressed: %d\n", cv);
    float x = 23.0f, y = 42.0f, z = 66.6f;
    unpackVector(cv, &x, &y, &z);
    printf("Result: %f, %f, %f\n", x, y, z);
}
