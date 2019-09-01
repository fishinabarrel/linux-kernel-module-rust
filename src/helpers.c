#include <linux/bug.h>
#include <linux/printk.h>
#include <linux/rcupdate.h>
#include <linux/uaccess.h>
#include <linux/version.h>
#include <linux/sched/signal.h>
#include <linux/sched/task.h>


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

void rcu_read_lock_helper(void) {
    rcu_read_lock();
}

void rcu_read_unlock_helper(void) {
    rcu_read_unlock();
}

struct task_struct *next_task_helper(struct task_struct *p) {
    return next_task(p);
}

void task_lock_helper(struct task_struct *p) {
    return task_lock(p);
}

void task_unlock_helper(struct task_struct *p) {
    return task_unlock(p);
}

/* see https://github.com/rust-lang/rust-bindgen/issues/1671 */
_Static_assert(__builtin_types_compatible_p(size_t, uintptr_t),
               "size_t must match uintptr_t, what architecture is this??");
