#include <linux/module.h>
#include <linux/fs.h>
#include <linux/slab.h>

// Bindgen gets confused at certain things
//
const gfp_t BINDINGS_GFP_KERNEL = GFP_KERNEL;
