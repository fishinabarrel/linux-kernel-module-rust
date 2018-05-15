#include <linux/printk.h>

int printk_helper(const unsigned char *s, int len)
{
	return printk(KERN_INFO "%.*s\n", len, (const char *)s);
}
