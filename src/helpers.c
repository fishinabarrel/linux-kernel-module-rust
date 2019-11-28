#include <linux/bug.h>
#include <linux/printk.h>
#include <linux/uaccess.h>
#include <linux/version.h>


int printk_helper(const unsigned char *s, int len)
{
    return printk(KERN_INFO "%.*s", len, (const char *)s);
}

void bug_helper(void)
{
    BUG();
}

int access_ok_helper(const void __user *addr, unsigned long n)
{
#if LINUX_VERSION_CODE >= KERNEL_VERSION(5, 0, 0) /* v5.0-rc1~46 */
    return access_ok(addr, n);
#else
    return access_ok(0, addr, n);
#endif
}

long __must_check PTR_ERR_helper(__force const void *ptr)
{
    return PTR_ERR(ptr);
}

bool __must_check IS_ERR_helper(__force const void *ptr)
{
	return IS_ERR(ptr);
}
