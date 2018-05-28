#include <linux/bug.h>
#include <linux/printk.h>

int printk_helper(const unsigned char *s, int len)
{
    return printk(KERN_INFO "%.*s", len, (const char *)s);
}

void bug_helper(void)
{
    BUG();
}
