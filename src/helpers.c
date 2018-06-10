#include <linux/bug.h>
#include <linux/printk.h>
#include <linux/uaccess.h>


int printk_helper(const unsigned char *s, int len)
{
    return printk(KERN_INFO "%.*s", len, (const char *)s);
}

void bug_helper(void)
{
    BUG();
}

int access_ok_helper(unsigned int mode, const void __user *addr, unsigned long n)
{
    return access_ok(mode, addr, n);
}
