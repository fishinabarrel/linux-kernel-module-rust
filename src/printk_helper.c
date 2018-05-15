#include <linux/printk.h>

int printk_helper(const unsigned char *s, int len)
{
	return printk("%.*s", len, (const char *)s);
}
