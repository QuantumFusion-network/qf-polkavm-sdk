import toast, { Toaster } from 'react-hot-toast';

export const useToast = () => {
  return {
    onSuccess: (msg) => toast.success(msg, {position: "top-right"}),
    onError: (msg) => toast.error(msg, {position: "top-right"}),
    toast,
    Toaster

  }
}
