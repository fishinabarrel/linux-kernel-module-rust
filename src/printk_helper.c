#include <linux/printk.h>

int printk_info_helper(const unsigned char *s, int len)
{
	return printk(KERN_INFO "%.*s", len, (const char *)s);
}

int printk_cont_helper(const unsigned char *s, int len)
{
	return printk(KERN_CONT "%.*s", len, (const char *)s);
}
