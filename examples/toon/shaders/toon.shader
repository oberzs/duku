vert.spv                                                                                                                    00000010554 00000000000 0005346                                                                                                      ustar                                                                                                                                                                                                                                                          #     p                 GLSL.std.450                      main       .   1   5   9   L   Y   ^   `   f   i   k        �   
 GL_GOOGLE_cpp_style_line_directive    GL_GOOGLE_include_directive      main      	   modelspace_position      in_modelspace_position       worldspace_position      Constants            model_matrix            albedo_index            sampler_index        object       screenspace_position          Light             coords           color     %   WorldObject   %       world_matrix      %      lights    %      camera_position   %      time      %      light_matrices    %      cascade_splits    %      variance_min      %      shadow_low    '   world     .   out_modelspace_position   1   out_worldspace_position  	 5   out_screenspace_position      9   out_lightspace_position   L   out_normal    Y   in_normal     ^   out_uv    `   in_uv     d   gl_PerVertex      d       gl_Position   d      gl_PointSize      d      gl_ClipDistance   d      gl_CullDistance   f         i   out_color     k   in_color      m   MaterialObject    m       arg_1     m      arg_2     m      arg_3     m      arg_4     m      arg_5     m      arg_6     m      arg_7     m      arg_8     o   material    G            H            H         #       H               H        #   @   H        #   D   G        H          #       H         #      G  #          G  $      @   H  %          H  %       #       H  %             H  %      #   @   H  %      #   �   H  %      #   �   H  %         H  %      #   �   H  %            H  %      #   �  H  %      #   �  H  %      #   �  G  %      G  '   "       G  '   !       G  .         G  1         G  5         G  9         G  L          G  Y         G  ^         G  `         H  d              H  d            H  d            H  d            G  d      G  i         G  k         H  m       #       H  m      #      H  m      #       H  m      #   0   H  m      #   @   H  m      #   P   H  m      #   `   H  m      #   p   G  m      G  o   "      G  o   !            !                                          
                  
   ;           +          �?                                           	      ;        	   +                  	                    !           +  !   "        #       "     $      "    
 %      #   
      $               &      %   ;  &   '         (            -      
   ;  -   .      ;  -   1         4         ;  4   5        7      "      8      7   ;  8   9      +     :      +     @      +     F      ;  -   L        Q   
      ;     Y        \            ]      \   ;  ]   ^         _      \   ;  _   `      +  !   b        c      b     d         c   c      e      d   ;  e   f      ;  4   i         j         ;  j   k       
 m                              n      m   ;  n   o      6               �     ;     	      ;           ;           =  
         Q               Q              Q              P                    >  	      A              =           =        	   �              >        A  (   )   '      =     *   )   =     +      �     ,   *   +   >     ,   =     /   	   O  
   0   /   /             >  .   0   =     2      O  
   3   2   2             >  1   3   =     6      >  5   6   A  (   ;   '   :      =     <   ;   =     =      �     >   <   =   A  4   ?   9      >  ?   >   A  (   A   '   :   @   =     B   A   =     C      �     D   B   C   A  4   E   9   @   >  E   D   A  (   G   '   :   F   =     H   G   =     I      �     J   H   I   A  4   K   9   F   >  K   J   A     M         =     N   M        O      "   N   T     P   O   Q     R   P       O  
   S   R   R             Q     T   P      O  
   U   T   T             Q     V   P      O  
   W   V   V             P  Q   X   S   U   W   =  
   Z   Y   �  
   [   X   Z   >  L   [   =  \   a   `   >  ^   a   =     g      A  4   h   f      >  h   g   =     l   k   >  i   l   �  8                                                                                                                                                      frag.spv                                                                                                                    00000026164 00000000000 0005311                                                                                                      ustar                                                                                                                                                                                                                                                          #     �                GLSL.std.450                     main    �   �   ;  @  �  �  �  �               �   
 GL_GOOGLE_cpp_style_line_directive    GL_GOOGLE_include_directive      main         linstep(f1;f1;f1;     	   low   
   high         v        tex_vsm(i1;vf2;f1;       index        uv       compare      tex_coord(i1;        index        Light            coords          color    
 "   shadow(struct-Light-vf4-vf41;     !   light     $   fragment(     2   moments   8   shadow_maps   A   samplers      K   p     Q   variance      \   Light     \       coords    \      color     `   WorldObject   `       world_matrix      `      lights    `      camera_position   `      time      `      light_matrices    `      cascade_splits    `      variance_min      `      shadow_low    b   world     h   d     m   low   w   p_max        param     �   param     �   param     �   coord     �   in_lightspace_position    �   uv    �   depth     �   depth     �   in_screenspace_position   �   blend_margin      �   cascade   �   coord     �   param     �   blend     �   param     �   param     �   param     �   shadow    �   param     �   param     �   param     �   next_cascade        next_coord      param       next_shadow     param       param       param       mesh_color      ambient_color       diffuse_color       specular_color    "  rim_color     #  glossiness    %  rim_amount    '  rim_threshold     (  light     0  light_dir     6  view_dir      ;  in_worldspace_position    ?  normal    @  in_normal     C  NdotL     G  diffuse_intensity     J  param     O  diffuse   S  half_vector   X  NdotH     \  specular_intensity    h  specular      l  rim_dot   q  rim_intensity     }  rim   �  out_color     �  MaterialObject    �      arg_1     �     arg_2     �     arg_3     �     arg_4     �     arg_5     �     arg_6     �     arg_7     �     arg_8     �  material      �  Constants     �      model_matrix      �     albedo_index      �     sampler_index     �  object    �  textures      �  in_uv     �  in_color      �  in_modelspace_position  G  8   "      G  8   !       G  A   "      G  A   !      H  \       #       H  \      #      G  ^          G  _      @   H  `          H  `       #       H  `             H  `      #   @   H  `      #   �   H  `      #   �   H  `         H  `      #   �   H  `            H  `      #   �  H  `      #   �  H  `      #   �  G  `      G  b   "       G  b   !       G  �         G  �         G  ;        G  @         G  �         H  �      #       H  �     #      H  �     #       H  �     #   0   H  �     #   @   H  �     #   P   H  �     #   `   H  �     #   p   G  �     G  �  "      G  �  !       H  �         H  �      #       H  �            H  �     #   @   H  �     #   D   G  �     G  �  "      G  �  !       G  �        G  �        G  �             !                             !                                                                !                            !                                             !            +     -       +     .     �? 	 3                              4           +  4   5        6   3   5      7       6   ;  7   8          :       3     =   +  4   >        ?   =   >      @       ?   ;  @   A       +     B         C       =     F   3   +  4   M       +  4   R        [           \         +  4   ]        ^   \   ]     _   [   ]    
 `   [   ^         _               a      `   ;  a   b      +     c         d         +     n         �           �      ]      �      �   ;  �   �         �         +     �      ?+  4   �      ;  �   �         �         +     �      +     �   ��L=  �   +     �       +     �      +     �         �         +       ���=+       ���>+       333?,             +       ���>,             ,       .   .   .   +        fff?,     !           +     $     B+     &  �K7?   )     \      7           :        ;  :  ;     ;  :  @     +     H  
�#<+     e  
ף;   �        ;  �  �      
 �                             �     �  ;  �  �       �  [            �  	   �  ;  �  �  	   +  4   �  d     �  3   �     �      �  ;  �  �         �        ;  �  �     ;  �   �     ;  :  �     6               �     9     �  $   �  8  6               7     	   7     
   7        �     =     &      =     '   	   �     (   &   '   =     )   
   =     *   	   �     +   )   *   �     ,   (   +        /      +   ,   -   .   �  /   8  6               7        7        7        �     ;     2      ;     K      ;     Q      ;     h      ;     m      ;     w      ;           ;     �      ;     �      =     9      A  :   ;   8   9   =  3   <   ;   A  C   D   A   B   =  =   E   D   V  F   G   <   E   =     H      W     I   G   H   O     J   I   I          >  2   J   =     L      A     N   2   M   =     O   N        P      0   L   O   >  K   P   A     S   2   R   =     T   S   A     U   2   M   =     V   U   A     W   2   M   =     X   W   �     Y   V   X   �     Z   T   Y   A  d   e   b   c   =     f   e        g      (   Z   f   >  Q   g   =     i      A     j   2   M   =     k   j   �     l   i   k   >  h   l   A  d   o   b   n   =     p   o   A  d   q   b   n   =     r   q   =     s      o     t   s   �     u   r   t   �     v   p   u   >  m   v   =     x   Q   =     y   Q   =     z   h   =     {   h   �     |   z   {   �     }   y   |   �     ~   x   }   =     �   m   >     �   >  �   .   >  �   ~   9     �         �   �   >  w   �   =     �   K   =     �   w        �      (   �   �        �      %   �   .   �  �   8  6               7        �     ;  �   �      ;     �      ;     �      =     �      A  �   �   �   �   =     �   �   >  �   �   A     �   �   R   =     �   �        �   �   A     �   �   R   >  �   �   =     �   �   O     �   �   �          A     �   �   5   =     �   �   P     �   �   �   �     �   �   �   �     �   �   �   P     �   �   �   �     �   �   �   >  �   �   A     �   �   �   =     �   �   A     �   �   5   =     �   �   �     �   �   �   >  �   �   A     �   �   M   =     �   �   A     �   �   R   =     �   �   =     �   �   P     �   �   �   �   �  �   8  6     "           7     !   �  #   ;     �      ;     �      ;     �      ;  �   �      ;     �      ;     �      ;     �      ;     �      ;     �      ;     �      ;     �      ;     �      ;     �      ;     �      ;  �        ;          ;          ;          ;          ;          A  �   �   �   �   =     �   �   >  �   �   A  d   �   b   �   �   =     �   �   �     �   �   �   >  �   �   =     �   �   A  d   �   b   �   M   =     �   �   �  �   �   �   �   �  �       �  �   �   �   �  �   >  �   �   �  �   �  �   =     �   �   A  d   �   b   �   R   =     �   �   �  �   �   �   �   �  �       �  �   �   �   �  �   >  �   �   �  �   �  �   >  �   �   �  �   �  �   �  �   �  �   =     �   �   >  �   �   9     �      �   >  �   �   A     �   �   �   =     �   �   �  �   �   �   .   �  �       �  �   �   �   �  �   �  -   �  �   =     �   �        �   �   =     �   �   =     �   �   A  d   �   b   �   �   =     �   �   �     �   �   �        �      1   �   -   �   >  �   �   =     �   �   �  �   �   �   -   �  �       �  �   �   �   �  �   =     �   �   >  �   �   =     �   �   O     �   �   �          >  �   �   A     �   �   �   =     �   �   >  �   �   9     �      �   �   �   �  �   �  �   =     �   �   >  �   �   =     �   �   O     �   �   �          >  �   �   A     �   �   �   =     �   �   >  �   �   9     �      �   �   �   >  �   �   =     �   �   �     �   �   �              '   �   �   >  �      =       �   >      9            >      =       �   >      =     	    O     
  	  	         >    
  A         �   =         >      9                >      =       �   =         =       �             .         �    �  �   �  �  �   �  8  6     $          �  %   ;  �        ;  �        ;  �        ;  �        ;  �   "     ;     #     ;     %     ;     '     ;     (     ;  �   0     ;  �   6     ;  �   ?     ;     C     ;     G     ;     J     ;  �   O     ;  �   S     ;     X     ;     \     ;  �   h     ;     l     ;     q     ;  �   }     >      >      >      >    !  >  "    >  #  $  >  %  &  >  '    A  )  *  b   �   �   =  \   +  *  Q     ,  +      A  �   -  (  �   >  -  ,  Q     .  +     A  �   /  (  �   >  /  .  A  �   1  (  �   =     2  1  O     3  2  2                 4  3       5     E   4  >  0  5  A  7  8  b   �   =     9  8  =     <  ;  �     =  9  <       >     E   =  >  6  >  =     A  @       B     E   A  >  ?  B  =     D  ?  =     E  0  �     F  D  E  >  C  F  =     I  C  =     K  (  >  J  K  9     L  "   J  �     M  I  L       N     1   -   H  M  >  G  N  =     P    =     Q  G  �     R  P  Q  >  O  R  =     T  0  =     U  6  �     V  T  U       W     E   V  >  S  W  =     Y  ?  =     Z  S  �     [  Y  Z  >  X  [  =     ]  X  =     ^  G  �     _  ]  ^  =     `  #  =     a  #  �     b  `  a       c        _  b       d     +   c  -   .   >  \  d  =     f  \       g     1   e  H  f  >  \  g  =     i    =     j  \  �     k  i  j  >  h  k  =     m  6  =     n  ?  �     o  m  n  �     p  .   o  >  l  p  =     r  l  =     s  C  =     t  '       u        s  t  �     v  r  u  >  q  v  =     w  %  �     x  w  H  =     y  %  �     z  y  H  =     {  q       |     1   x  z  {  >  q  |  =     ~  "  =       q  �     �  ~    >  }  �  =     �    =     �    =     �  O  �     �  �  �  =     �  h  �     �  �  �  =     �  }  �     �  �  �  �     �  �  �  Q     �  �      Q     �  �     Q     �  �     P     �  �  �  �  .   >  �  �  �  8                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              